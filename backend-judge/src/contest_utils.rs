use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive; // zip 0.5.13

fn build_req_map(probs: &i32, nums_tests: &Vec<i32>) -> HashMap<String, bool>{
    let mut m: HashMap<String, bool> = HashMap::new();
    let mut problems = false;
    let num_probs = *probs;
    for i  in 1..=num_probs {
        m.insert(format!("problem_{}/", i), false);
        m.insert(format!("problem_{}/problem.md", i), false);
        m.insert(format!("problem_{}/solution.c",i), false);
        m.insert(format!("problem_{}/testcases/",i), false);
        let x = nums_tests[(i - 1) as usize];
        for j in 1..=x{
            m.insert(format!("problem_{}/testcases/input_{}.txt", i, j), false);
        }
    }


    return m;
}


pub fn checker(name: &str, num_probs: &i32, nums_tests: &Vec<i32>) -> Result<(), String>{

    // check for a folder named problems at top level

    let mut m = build_req_map(num_probs, nums_tests);

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


pub fn save_zip(file_path: &str, save_file_name: &str) -> Result<(), String>{
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
