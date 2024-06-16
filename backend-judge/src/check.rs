use std::error::Error;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive; // zip 0.5.13

fn checker(archive: ZipArchive<File>, num_probs: i32, nums_tests: Vec<i32>) -> bool{

    // check for a folder named problems at top level
    let mut problems = false;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.enclosed_name().unwrap();
        if name.to_string_lossy() == "problems/" {
            problems = true;
            break;
        }
    }

    if !problems {
        return false;
    }

    // check for the number of problems
    let mut count = 0;


    true
}
fn main() -> Result<(), Box<dyn Error>> {
    let archive = File::open("/home/dikshant/Downloads/f.zip")?;
    let mut archive = ZipArchive::new(archive)?;

    // iterate over all files, because you don't know the exact name
    for idx in 0..archive.len() {
        let mut entry = archive.by_index(idx)?;
        let name = entry.enclosed_name();


        // println!("Entry: {:?}", name.unwrap());
        if let Some(name) = name {
            let n = name.to_str().unwrap();
            println!("Entry: {:?}", n );
            let v = n.split("/");
            println!("Count: {:?}", v );

            // check if the entry is a file
            if name.to_string_lossy().ends_with(".txt") {
                let mut x : String= "".to_string();
                entry.read_to_string(&mut x).expect("TODO: panic message");
                println!("Content: {}", x);

            }
        }
    }

    Ok(())
}