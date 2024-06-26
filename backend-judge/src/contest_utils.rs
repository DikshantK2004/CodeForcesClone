use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use diesel::{PgConnection, RunQueryDsl};
use uuid::Uuid;
use zip::ZipArchive;
use crate::models::{Problem, SampleTestCase}; // zip 0.5.13

fn build_req_map(probs: &i32, nums_tests: &Vec<i32>, nums_samples: &Vec<i32>) -> HashMap<String, bool>{
    let mut m: HashMap<String, bool> = HashMap::new();
    let problems = false;
    let num_probs = *probs;
    for i  in 1..=num_probs {
        m.insert(format!("problem_{}/", i), false);
        m.insert(format!("problem_{}/problem.md", i), false);
        m.insert(format!("problem_{}/solution.c",i), false);
        m.insert(format!("problem_{}/testcases/",i), false);
        m.insert(format!("problem_{}/samples/",i), false);
        let mut x = nums_tests[(i - 1) as usize];
        for j in 1..=x{
            m.insert(format!("problem_{}/testcases/input_{}.txt", i, j), false);
        }

        x = nums_samples[(i - 1) as usize];

        for j in 1..=x{
            m.insert(format!("problem_{}/samples/sample_{}.txt", i, j), false);
            m.insert(format!("problem_{}/samples/answer_{}.txt", i, j), false);
        }

    }


    return m;
}


pub fn checker(name: &str, num_probs: &i32, nums_tests: &Vec<i32>, nums_samples: &Vec<i32>) -> Result<(), String>{

    // check for a folder named problems at top level

    let mut m = build_req_map(num_probs, nums_tests, nums_samples);

    let archive = File::open(name).unwrap();
    let mut archive = ZipArchive::new(archive).unwrap();

    for idx in 0..archive.len() {
        let entry = archive.by_index(idx).unwrap();
        let name = entry.enclosed_name();


        // println!("Entry: {:?}", name.unwrap());
        if let Some(name) = name {
            let n = name.to_str().unwrap();
            if m.contains_key(n){
                m.insert(n.to_string(), true);
            }
            else {
                return Err(format!("File: {} not allowed", n));
            }
        }
    }

    // if map contains some false values then return error
    for (k, v) in m.iter(){
        if !v {
            return Err(format!("File: {} not found", k));
        }
    }


    Ok(())
}


pub fn extract_zip(file_path: &str, save_file_name: &str) -> Result<(), String>{
    let output = std::process::Command::new("unzip")
        .arg(file_path)
        .arg("-d")
        .arg(String::from("./data/") + save_file_name)
        .output()
        .expect("failed to execute process");

    if !output.status.success(){
        // remove if something was unzipped
        std::fs::remove_dir_all(String::from("./data/") + save_file_name).expect("Couldn't  remove directory");
        return Err(String::from("Error unzipping file"));
    }

    Ok(())
}


pub fn remove_zip(file_path: &str) -> Result<(), String>{
    let output = std::process::Command::new("rm")
        .arg(file_path)
        .output()
        .expect("failed to execute process");

    if !output.status.success(){
        return Err(String::from("Error removing file"));
    }

    Ok(())
}


pub fn remove_existing_contest(contest_id: &str) -> Result<(), String>{
    let output = std::process::Command::new("rm")
        .arg("-r")
        .arg(String::from("./data/") + contest_id)
        .output()
        .expect("failed to execute process");

    if !output.status.success(){
        return Err(String::from("Error removing contest"));
    }

    Ok(())
}

pub fn insert_problems_to_db(num_problems: i32, prob_names: Vec<String>, num_tests: Vec<i32>, num_samples: Vec<i32>, contest_id: String, connection: &mut PgConnection){
    for i in 0..num_problems{
        let new_problem_id = Uuid::new_v4().to_string();
        let problem = Problem{
            problem_num: i+1,
            name: prob_names[i as usize].clone(),
            num_tests: num_tests[i as usize],
            contest_id: contest_id.clone(),
            num_samples: num_samples[i as usize]
        };

        diesel::insert_into(crate::schema::problems::table)
            .values(&problem)
            .execute(connection)
            .expect("Error saving problem");
    }
}

pub fn gather_samples(contest_id: &str, problem_num: i32, num_samples: i32) -> Result<Vec<SampleTestCase>, String>{
    let mut samples = Vec::new();
    for i in 0..num_samples{
        let input_file_path = Path::new("./data/").join(contest_id).join(format!("problem_{}/samples/sample_{}.txt", problem_num, i + 1));
        let output_file_path = Path::new("./data/").join(contest_id).join(format!("problem_{}/samples/answer_{}.txt", problem_num, i + 1));
        let input_data = std::fs::read_to_string(input_file_path);
        let output_data = std::fs::read_to_string(output_file_path);

        if let Err(e) = input_data{
            return Err("Error Reading samples".to_string());
        }

        if let Err(e) = output_data{
            return Err("Error Reading samples".to_string());
        }

        samples.push(SampleTestCase{
            input: input_data.unwrap(),
            output: output_data.unwrap()
        });
    }

    Ok(samples)
}