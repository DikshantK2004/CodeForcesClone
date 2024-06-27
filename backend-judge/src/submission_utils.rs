use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use diesel::{QueryDsl, RunQueryDsl};
use crate::database::establish_connection;
use crate::models::{NewSubmission, NewTestResult, Submission};
use crate::schema::{submissions, test_results};


enum Verdicts{
    AC,
    WA(i32),
    TLE,
    MLE,
    SF // server failed
}

fn save_code_file(code: &str, extension: &str, submission_id: i32) -> Result<String, String>{
    let time = chrono::Local::now().naive_local();
    let save_file_name_without_extension = format!("{}", submission_id);
    let save_file_name = format!("{}.{}", save_file_name_without_extension, extension);
    let mut f = File::create(format!("./media/{}", save_file_name));
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


fn run_file(file_name: &str, extension: &str, test_file: &str, test_num: i32) -> Result<(String, String), String>{
    let input_file = File::open(test_file)?;
    let mut expected_output = String::new();
    let mut file = File::open(file_name)?;
    let command = get_command_to_run(file_name, extension)?;

    let output = Command::new(command)
        .stdin(input_file)
        .output()?;
    let code_without_ext = file_name.replace(&format!(".{}", &extension), "");

    let user_output = String::from_utf8_lossy(&output.stdout).to_string();
    let output_path = format!("./media/{}_{}.txt", code_without_ext, test_num );
    let mut output_file = std::fs::File::create(output_path.as_str())?;
    output_file.write_all(user_output.as_bytes())?;
    Ok((user_output, output_path))
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
            submission_id,
            test_num: i as i32 + 1,
            out: out.clone(),
            verdict: verdict.clone(),
        })
        .collect();

    // Insert all the test results in one query
    diesel::insert_into(test_results::table)
        .values(&new_test_results)
        .execute(connection)?;

    // updating the submission with new verdict
    let final_verdict = if len == num_tests && verdicts.iter().all(|verdict| verdict == "YES") {
        "Accepted"
    }
    else {
        verdicts[len -1].as_str()
    };

    diesel::update(submissions::table.find(submission.id))
        .set(submissions::verdict.eq(final_verdict))
        .execute(connection)?;

    Ok(())
}

// TODO add parallelism with threads
fn validate(submission: &Submission, contest_id: &str, problem_num: i32, num_tests: i32) -> Result<(), String>{

    let execute_validator_command = format!("./data/{}/problem_{}/validator.out", contest_id, problem_num);
    let input_file_name = save_code_file(&submission.code, &submission.extension, submission.id)?;

    let results = Vec::new();
    let mut outputs = Vec::new();
    let mut verdicts = Vec::new();
    for i in 1..=num_tests{
        let test_file = format!("./data/{}/problem_{}/testcases/input_{}.txt", contest_id, problem_num, i);
        let( user_output,output_path) = run_file(&input_file_name, &submission.extension, &test_file, i)?;

        let mut child = Command::new(execute_validator_command.clone())
            .args(&[output_path.as_str()])
            .stdin(test_file.as_str())
            .output();
        outputs.push(user_output);
        let mut validator_out = match child{
            Ok(output) => {
                String::from_utf8_lossy(&output.stdout).to_string()
            },
            Err(e) => {
                return Err(format!("System Error"));
            }
        };

        verdicts.push(validator_out);
        if validator_out != "YES"{
            break;
        }
    }

    let truncated_outputs = truncate_outputs(outputs, 1497);
    let truncated_verdicts = truncate_verdicts(verdicts, 40);

    store_results_to_db(submission, truncated_outputs, truncated_verdicts, num_tests)?;

    Ok(())
}
