use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use diesel::{QueryDsl, RunQueryDsl};
use crate::database::establish_connection;
use crate::models::{NewSubmission, NewTestResult, Submission};
use crate::schema::{submissions, test_results};
use diesel::expression_methods::ExpressionMethods; // IMPORANT: for eq


enum Verdicts{
    AC,
    WA(i32),
    TLE,
    MLE,
    SF // server failed
}

enum TypeOnRun{
    Err(String),
    Out(String),
    Out2(String, String)
}

fn save_code_file(code: &str, extension: &str, submission_id: i32) -> Result<String, String>{
    let time = chrono::Local::now().naive_local();
    let save_file_name_without_extension = format!("{}", submission_id);
    let save_file_name = format!("./media/{}.{}", save_file_name_without_extension, extension);
    let mut f = File::create(save_file_name.as_str());
    if let Err(e) = f {
        return Err(format!("Failed to create file: {}", e));
    }
    let mut file = f.unwrap();

    let res = file.write_all(code.as_bytes());
    if let Err(e) = res {
        return Err(format!("Failed to write to file: {}", e));
    }
    res.unwrap();

    Ok(save_file_name)

}


fn get_command_to_run(file_name: &str, extension: &str) -> Result<String, String>{
    match extension{
        "py" => Ok(format!("python {}", file_name)),
        "c" => {
            let output_file = file_name.replace(".c", ".out");
            let compile_command = format!("gcc {} -o {}", file_name, output_file);
            let run_command = format!("./{}", output_file);
            Ok(compile_command + " && " + &*run_command)
        },
        "cpp" => {
            let output_file = file_name.replace(".cpp", ".out");
            let compile_command = format!("g++ {} -o {}", file_name, output_file);
            let run_command = format!("./{}", output_file);
            Ok(compile_command + " && " + &*run_command)
        },
        _ => Err("Invalid extension".to_string())
    }
}


fn run_file(file_name: &str, extension: &str, test_file: &str, test_num: i32) -> Result<TypeOnRun, String>{
    println!("Test file: {}", test_file);
    let input_file = match File::open(test_file) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let command = get_command_to_run(file_name, extension)?;
    println!("{}", command);
    let output = match Command::new("sh")
        .args(&["-c",command.as_str()])
        .stdin(input_file)
        .output() {
        Ok(output) => output,
        Err(e) => return Err(e.to_string()),  // Convert the std::io::Error into a String
    };

    let code_without_ext = file_name.replace(&format!(".{}", &extension), "");

    let stan_err = String::from_utf8_lossy(&output.stderr).to_string();
    if stan_err.len() > 0 {
        return Ok(TypeOnRun::Err(stan_err));
    }
    let user_output = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Output on test_num: {} is: {}", test_num, user_output);

    let output_path = format!("{}_{}.txt", code_without_ext, test_num );
    println!("Output path: {}", output_path);
    let mut output_file = match File::create(output_path.as_str()) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()), // Convert the error to a String
    };

    println!("Output path: {}", output_path);



    // Attempt to write to the file
    if let Err(e) = output_file.write_all(user_output.as_bytes()) {
        return Err(e.to_string());
    };
    Ok(TypeOnRun::Out2(user_output, output_path))
}


fn truncate_outputs(outputs: Vec<String>, max_length: usize) -> Vec<String> {
    outputs.iter().map(|output| {
        if output.len() > max_length {
            format!("{}...", &output[..max_length])
        } else {
            output.clone()
        }
    }).collect()
}

fn truncate_verdicts(verdicts: Vec<String>, max_length: usize) -> Vec<String> {
    verdicts.iter().map(|verdict| {
        if verdict.len() > max_length {
            verdict[..max_length].to_string()
        } else {
            verdict.clone()
        }
    }).collect()
}

fn store_results_to_db(submission: &Submission, outputs: Vec<String>, verdicts: Vec<String>, num_tests: i32) -> Result<(), String>{
    let connection = & mut establish_connection();
    let len = outputs.len() as i32;
    let new_test_results: Vec<NewTestResult> = outputs.iter()
        .zip(verdicts.iter())
        .enumerate()
        .map(|(i , (out, verdict))| NewTestResult {
            submission_id: submission.id,
            test_num: i as i32 + 1,
            out: out.clone(),
            verdict: verdict.clone(),
        })
        .collect();

    // Insert all the test results in one query
    let mut res  = diesel::insert_into(test_results::table)
        .values(&new_test_results)
        .execute(connection);
    if let Err(e) = res{
        return Err(e.to_string());
    }
    // updating the submission with new verdict
    let final_verdict = if len == num_tests && verdicts.iter().all(|verdict| verdict == "YES") {
        "Accepted"
    }
    else if verdicts[len as usize - 1].as_str() == "NO"{
        "Wrong Answer"
    }
    else{ "Failed System Testing" };

    res = diesel::update(submission)
        .set(submissions::dsl::verdict.eq(final_verdict))
        .execute(connection);

    if let Err(e) = res{
        return Err(e.to_string());
    }

    Ok(())
}

// TODO add parallelism with threads
fn validate(submission: &Submission, contest_id: &str, problem_num: i32, num_tests: i32) -> Result<(), String>{

    let execute_validator_command = format!("./data/{}/problem_{}/validator.out ", contest_id, problem_num);
    let input_file_name = save_code_file(&submission.code, &submission.extension, submission.id)?;
    println!("Input file name: {}", input_file_name);
    let mut outputs = Vec::new();
    let mut verdicts = Vec::new();
    for i in 1..=num_tests{
        println!("Test started: {}", i);
        let test_file_path = format!("./data/{}/problem_{}/testcases/input_{}.txt", contest_id, problem_num, i);
        println!("Test file path: {}", test_file_path);

        // running the file
        let( user_output,output_path)  : (String, String) = match run_file(&input_file_name, &submission.extension, &test_file_path, i)?{
            TypeOnRun::Err(e) => {
                verdicts.push(format!("Error on test {}", i));
                outputs.push(e);
                break;
            },
            TypeOnRun::Out2(s1, s2) => (s1, s2),
            _ => ("".to_string(), "".to_string())
        };

        let test_file = match std::fs::File::open(test_file_path.as_str()) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };
        let mut child = Command::new("sh")
            .args(&["-c" , &*(execute_validator_command.clone() + &*output_path)])
            .stdin(test_file)
            .output();
        outputs.push(user_output);
        let mut validator_out = match child{
            Ok(output) => {
                String::from_utf8_lossy(&output.stderr).to_string() + &*String::from_utf8_lossy(&output.stdout).to_string()
            },
            Err(e) => {
                return Err(format!("System Error"));
            }
        };
        println!("Validator output: {}", validator_out);

        verdicts.push(validator_out.clone());
        if validator_out != "YES"{
            break;
        }
    }

    let truncated_outputs = truncate_outputs(outputs, 1497);
    let truncated_verdicts = truncate_verdicts(verdicts, 40);

    store_results_to_db(submission, truncated_outputs, truncated_verdicts, num_tests)?;

    Ok(())
}

pub fn run_tests(submission: &Submission, contest_id: &str, problem_num: i32, num_tests: i32) -> (){

    validate(submission, contest_id, problem_num, num_tests).unwrap_or_else(|e|{
        println!("Error validating submission: {:?}", e);
    });
}
