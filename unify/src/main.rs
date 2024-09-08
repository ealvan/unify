// use std::{fs, io};

// fn main() -> io::Result<()> {
//     let mut entries = fs::read_dir("../test/r1")?
//         .map(|res| res.map(|e| e.path()))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     // The order in which `read_dir` returns entries is not guaranteed. If reproducible
//     // ordering is required the entries should be explicitly sorted.

//     entries.sort();
//     for e in entries {
//         println!("{}", e.display());
//     }
//     // The entries have now been sorted by their path.

//     Ok(())
// }

//------------------------------------------------------------------
use std::io;
use std::fs::{self};
use std::path::Path;
use std::collections::HashSet;

fn get_files(dir: &Path, v: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir(){
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                get_files(&path, v);
            }else {
                // println!("{}", path.display());
                v.push(String::from(path.to_str().unwrap()));                
            }
        }
    }
    Ok(())
    /*
        d = getdir
        v : vect
        for e in d{
            if e.is_dir(){
                d = getdir
            }else{
                v.push(e)
            }
        }
    */
}

fn main() -> io::Result<()> {
    let mut r1: Vec<String> = Vec::new();
    let mut r2: Vec<String> = Vec::new();

    

    let rpath_r1 = Path::new("../test/r2");
    let rpath_r2 = Path::new("../test/r1");

    match get_files(&rpath_r1, &mut r1) {
        Ok(_) => println!("Succesfully worked!"),
        Err(why) => println!("The error is: {}", why),
    };
    match get_files(&rpath_r2, &mut r2) {
        Ok(_) => println!("Succesfully worked!"),
        Err(why) => println!("The error is: {}", why),
    };
    println!("R1");
    for f in r1{
        println!("{}",f);
    }
    println!("R2");
    for f in r2{
        println!("{}",f);
    }

    Ok(())
}