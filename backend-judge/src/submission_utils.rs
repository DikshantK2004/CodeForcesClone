use std::fs::File;
use std::io::Write;
use std::process::{Command};
use std::sync::mpsc;
use std::time::Instant;
use diesel::{ RunQueryDsl};
use crate::database::establish_connection;
use crate::models::{ NewTestResult, Problem, Submission};
use crate::schema::{submissions, test_results};
use diesel::expression_methods::ExpressionMethods;
use itertools::Itertools;
use crate::responses::LeaderboardCell; // IMPORANT: for eq


enum Verdicts{
    AC,
    WA,
    TLE,
    MLE,
    SF // system failed
}

enum CommandTime{
    In(i32, String, String), // time in ms, output, stderr
    Out( ) // TIME LIMIT EXCEEDED
}

enum TypeOnRun{
    Err( String), // if standard error exists
    TLE(),
    Out2(i32, String, String) // for returning output path and output
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

fn run_command_with_timeout(com: &str, input_file: File, time_limit:  i32) -> Result<CommandTime, String>{
    let com_str = com.to_string();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let start = Instant::now();
        let output = Command::new("sh")
            .args(&["-c",com_str.as_str()])
            .stdin(input_file)
            .output();

        match output {
            Ok(output) => {
                let duration = start.elapsed();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                let result = Ok(CommandTime::In(duration.as_millis() as i32, stdout, stderr));

                let _ = tx.send(result);
            },
            Err(e) => {
                let _ = tx.send(Err(e.to_string()));
            },
        }
    });

    match rx.recv_timeout(std::time::Duration::from_millis(time_limit as u64)) {
        Ok(res) => res,
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(CommandTime::Out()),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err("Channel disconnected before timeout".to_string()),
    }
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


fn run_file(file_name: &str, extension: &str, test_file: &str, test_num: i32, time_limit: i32) -> Result<TypeOnRun, String>{
    println!("Test file: {}", test_file);
    let input_file = match File::open(test_file) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let command = get_command_to_run(file_name, extension)?;
    println!("{}", command);

    let output = run_command_with_timeout(command.as_str(), input_file, time_limit);
    let (time, user_output, stderr) = match output{
        Ok(output) => match output {
            CommandTime::In(time, user_output, stderr) => (time, user_output, stderr),
            CommandTime::Out() => return Ok(TypeOnRun::TLE()),
        }
        Err(e) => return Err(e),
    };


    let code_without_ext = file_name.replace(&format!(".{}", &extension), "");
    if stderr.len() > 0 {
        return Ok(TypeOnRun::Err(stderr));
    }
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
    Ok(TypeOnRun::Out2(time,user_output, output_path))
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

fn store_results_to_db(submission: &Submission, outputs: Vec<String>, verdicts: Vec<String>, times: Vec<i32>, num_tests: i32) -> Result<(), String>{
    let connection = & mut establish_connection();
    let len = outputs.len() as i32;
    let new_test_results: Vec<NewTestResult> = outputs.iter()
        .zip(verdicts.iter())
        .zip(times.iter())
        .enumerate()
        .map(|(i , ((out, verdict), time))| NewTestResult {
            submission_id: submission.id,
            test_num: i as i32 + 1,
            out: out.clone(),
            verdict: verdict.clone(),
            time_taken: *time,
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
    let final_verdict:String = if len == num_tests && verdicts.iter().all(|verdict| verdict == "YES") {
        "Accepted".to_string()
    }
    else if verdicts[len as usize - 1].as_str() == "NO" {
        format!("Wrong Answer on test {}", len)
    }
    else if verdicts[len as usize - 1].as_str() == "TLE" {
        format!("Time Limit Exceeded on test {}", len)
    }
    else{
        "Failed System Testing".to_string()
    };
    let sub_time = Some(times.iter().sum::<i32>() / num_tests);
    res = diesel::update(submission).set((
        submissions::dsl::verdict.eq(final_verdict),
        submissions::dsl::time_taken.eq(sub_time),
    )).execute(connection);

    if let Err(e) = res{
        return Err(e.to_string());
    }

    Ok(())
}

// TODO add parallelism with threads
fn validate(submission: &Submission, contest_id: &str, problem_num: i32, num_tests: i32, time_limit:i32) -> Result<(), String>{

    let execute_validator_command = format!("./data/{}/problem_{}/validator.out ", contest_id, problem_num);
    let input_file_name = save_code_file(&submission.code, &submission.extension, submission.id)?;
    println!("Input file name: {}", input_file_name);
    let mut outputs = Vec::new();
    let mut verdicts= Vec::new();
    let mut times= Vec::new();
    for i in 1..=num_tests{
        println!("Test started: {}", i);
        let test_file_path = format!("./data/{}/problem_{}/testcases/input_{}.txt", contest_id, problem_num, i);
        println!("Test file path: {}", test_file_path);

        // running the file
        let( time,user_output,output_path)  : (i32,String, String) = match run_file(&input_file_name, &submission.extension, &test_file_path, i, time_limit)?{
            TypeOnRun::Err(e) => {
                verdicts.push(format!("Error on test {}", i));
                outputs.push(e);
                times.push(0);
                break;
            },
            TypeOnRun::TLE() =>{
                verdicts.push("TLE".to_string());
                outputs.push("".to_string());
                times.push(10);
                break;
            }
            TypeOnRun::Out2(time,s1, s2) => (time,s1, s2),
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
        times.push(time);
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

    store_results_to_db(submission, truncated_outputs, truncated_verdicts,times, num_tests)?;

    Ok(())
}

pub fn run_tests(submission: Submission, problem: Problem) -> (){
    let contest_id = problem.contest_id;
    let problem_num = problem.problem_num;
    let num_tests = problem.num_tests;
    let time_limit = problem.time_limit;
    validate(&submission, &*contest_id, problem_num, num_tests, time_limit).unwrap_or_else(|e|{
        println!("Error validating submission: {:?}", e);
    });
}


pub fn group_by_user_id(cells: Vec<LeaderboardCell>) -> Vec<(i32, Vec<LeaderboardCell>)>{
    let grouped_cells= cells.into_iter()
        .sorted_by_key(|cell |  cell.user_id)
        .chunk_by(|cell| cell.user_id)
        .into_iter()
        .map(|(user_id, group)| (user_id, group.collect()))
        .collect();
    grouped_cells
}

pub fn group_by_problem_id(cells: Vec<LeaderboardCell>) -> Vec<(i32, Vec<LeaderboardCell>)>{
    let grouped_cells= cells.into_iter()
        .sorted_by_key(|cell |  cell.problem_id)
        .chunk_by(|cell| cell.problem_id)
        .into_iter()
        .map(|(problem_id, group)| (problem_id, group.collect()))
        .collect();
    grouped_cells
}