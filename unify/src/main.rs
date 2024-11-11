use std::error::Error;
use std::fmt::format;
#[warn(unused_variables)]

use std::io;
use std::fs::{self, read_to_string, OpenOptions};
use std::os::linux::fs::MetadataExt;
use std::thread::current;

// use rayon::prelude::*;
use log::{info, error};
use fern::{meta, Dispatch};
use std::collections::HashSet;
use std::path::{self, Path};
use std::io::Write;
use std::fs::{DirBuilder,Metadata};

const ROOT1: &str = "r1"; 
const ROOT2: &str = "r2"; 
const ROOT3: &str = "r3"; 
const INTERSECTION:&str = "root1_and_root2.log";
const R1_R2:&str = "root1-root2.log";
const R2_R1:&str = "root2-root1.log";


fn get_files(dir: &Path, v: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            match get_files(&path, v) {
                Ok(_) => (),//println!("Succesfully worked!")
                Err(why) => error!("ERROR in get_files() - {}, data: {}",why, &path.display()),
            };
        }else {
            // println!("{}", path.display());
            v.push(String::from(path.to_str().unwrap()));
        }
    }
    Ok(())
}
fn print_vector(rootn: &Vec<String>){
    for f in rootn{
        println!("{}",f);
    }
}
// fn write_rootn_files(rootn:&Vec<String>, root_log_name: &str) -> std::io::Result<()>{
//     let mut content: String = rootn.join("\n");
//     let mut f = OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open(root_log_name)
//         .expect("error opening a file");

//     f.write_all(content.as_bytes()).expect("Unable to write");
//     Ok(())
// }
fn write_rootn_files(rootn:&HashSet<String>, root_log_name: &str) -> std::io::Result<()>{
    let mut content: String = itertools::join(rootn, "\n");
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(root_log_name)
        .expect("error opening a file");

    f.write_all(content.as_bytes()).expect("Unable to write");
    Ok(())
}

fn remove_prefix_in_place(myvec: &mut Vec<String>, prefix: &str){
    *myvec = myvec.iter()
              .filter_map(|entry| entry.strip_prefix(prefix).map(|e| e.to_string()))
              .collect();
}

fn store_files(fr1: Vec<String>, fr2: Vec<String>){
    let root1_set: HashSet<_> = fr1.into_iter().collect();
    let root2_set: HashSet<_> = fr2.into_iter().collect();

    let common: HashSet<_> = root1_set.intersection(&root2_set).cloned().collect();
    write_rootn_files(&common, &INTERSECTION);
    let dif1: HashSet<_> = root1_set.difference(&root2_set).cloned().collect();
    write_rootn_files(&dif1, &R1_R2);
    let dif2: HashSet<_> = root2_set.difference(&root1_set).cloned().collect();
    write_rootn_files(&dif2, &R2_R1);
}

fn create_dir(dir_path: &str) -> Result<(),io::Error>{
    fs::create_dir(dir_path)?;
    Ok(())
}

fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
    fs::rename(source, destination)?; // Move the file from source to destination
    println!("source: {} -- destination: {}", source, destination);
    Ok(()) // Return Ok if the operation completes successfully
}
fn delete_file(file_path: &str) -> Result<(), io::Error> {
    
    fs::remove_file(file_path)?; // Delete the file at the specified path
    println!("Remove file: {} ... OK", file_path);
    Ok(()) // Return Ok if the operation completes successfully
}

fn create_dir_path(path:&str){
    DirBuilder::new().recursive(true).create(path).unwrap();
}

fn move_files(from: &str, to: &str, prefix: &str){
    /*
        from: file where there is .log with files
        to: directory to place it
    */
    let mut current_file:String = String::from("");
    let mut destination :String = String::from("");
    let mut tmp_line: String = String::from("");
    for line in read_to_string(from).unwrap().lines(){
        tmp_line = line.to_string();

        current_file = tmp_line.clone();
        current_file = prefix.to_string() + &current_file;

        let tmp_line:&str = tmp_line.as_str();
        destination = format!("{to}{tmp_line}");//r3 + /original/path/line.txt

        let exists = Path::new(&current_file).exists();
        if exists{
            println!("file: {} exists: {}", current_file, exists);
            println!("destination: {}", &destination);
            
            let mut pdest: &Path = Path::new(&destination);
            pdest = pdest.parent().unwrap();

            if ! pdest.exists() {
                let pdest = pdest.display().to_string();            
                create_dir_path(&pdest);        
            }
            match move_file(&current_file, &destination) {
                Ok(_) => println!("move_file OK"),
                Err(why) => println!("{}",why)
            };
        }else{
            println!("file: {} doesnt exist", current_file);
        }
    }

}
fn get_filesize_and_lastupdate(file_path: &str) -> Result<(u64,i64), io::Error>{
    let metadata = fs::metadata(&file_path)?;
    Ok((metadata.len(), metadata.st_atime()))
}

fn do_versions(intersection_file: &str){
    let mut root1_file = String::from(ROOT1);
    let mut root2_file = String::from(ROOT2);
    let mut root3_file = String::from(ROOT3);

    for line in read_to_string(intersection_file).unwrap().lines(){
        root1_file.push_str(&line);
        root2_file.push_str(&line);
        let r1_exists: &Path = Path::new(&root1_file);
        let r2_exists: &Path = Path::new(&root2_file);
        let r1_exists = r1_exists.exists();
        let r2_exists = r2_exists.exists();
        if r1_exists & r2_exists{
            let (fr1, lacc1) = match get_filesize_and_lastupdate(&root1_file) {
                Ok(val) => (val.0, val.1),
                Err(why) => panic!("{}",why)
            };
            let (fr2, lacc2) = match get_filesize_and_lastupdate(&root2_file) {
                Ok(val) => (val.0, val.1),
                Err(why) => panic!("{}",why)
            };
            root3_file.push_str(&line);
            let path_version = Path::new(&root3_file);
            let parent_path = path_version.parent().unwrap().to_str().unwrap();
            let parent_path_ = Path::new(&parent_path);
            if ! parent_path_.exists(){
                let pdest = parent_path_.display().to_string();
                create_dir_path(&pdest);
            }
            if fr1 == fr2{
                // delete_file(&)
                println!("== > File r1: {}\nFile r2: {} size: {}", &root1_file,&root2_file, &fr1);
                println!("lacc1: {} lacc2: {}",lacc1, lacc2);

                if lacc1 > lacc2{
                    //delete file root2
                    delete_file(&root2_file).expect("Cannot delete r2 file");
                    match move_file(&root1_file, &root3_file) {
                        Ok(_) => println!("move_file OK"),
                        Err(why) => println!("{}",why)
                    };
                }else{
                    //delete file root1
                    delete_file(&root1_file).expect("Cannot delete r1 file");
                    match move_file(&root2_file, &root3_file) {
                        Ok(_) => println!("move_file OK"),
                        Err(why) => println!("{}",why)
                    };
                }
            }else{
                println!("!= > File r1: {}\nFile r2: {} fs1:{} fs2:{}", &root1_file, &root2_file, &fr1, &fr2);
                let filename = path_version.file_name().unwrap().to_str().unwrap();
                let new_version_path = format!("{parent_path}/v1_{filename}");
                
                match move_file(&root1_file, &new_version_path) {
                    Ok(_) => println!("move_file OK"),
                    Err(why) => println!("{}",why)
                };
                let new_version_path = format!("{parent_path}/v2_{filename}");

                match move_file(&root2_file, &new_version_path) {
                    Ok(_) => println!("move_file OK"),
                    Err(why) => println!("{}",why)
                };
            }

        }
    }
}

fn main(){
    println!("FIRST PHASE");
    //define root1, root2 & root3
    let root1 = Path::new(&ROOT1);    
    let root2 = Path::new(&ROOT2);
    let root3 = Path::new(&ROOT3);    
    //define containers data
    //fr1 -> Files from Root 1
    let mut fr1: Vec<String> = Vec::new();
    let mut fr2: Vec<String> = Vec::new();
    //get_files from root1 & root2
    match get_files(&root1, &mut fr1){
        Ok(_) => println!("Succesfully get root1's files"),
        Err(why) => panic!("The error is: {}", why)
    }
    match get_files(&root2, &mut fr2){
        Ok(_) => println!("Succesfully get root2's files"),
        Err(why) => panic!("The error is: {}", why)
    }
    
    print_vector(&fr1);
    print_vector(&fr2);

    //Now write the files in a persistent .log file
    // let root1_logname = "root1_file.log";
    // let root2_logname = "root2_file.log";
    let mut prefix = root1.display().to_string();    
    remove_prefix_in_place(&mut fr1, &prefix);
    prefix = root2.display().to_string();
    remove_prefix_in_place(&mut fr2, &prefix);

    // write_rootn_files(&fr1, &root1_logname);
    // write_rootn_files(&fr2, &root2_logname);
    store_files(fr1,fr2); 
    create_dir(&ROOT3);
    move_files(&R1_R2, &ROOT3, &ROOT1);
    move_files(&R2_R1, &ROOT3, &ROOT2);
    do_versions(&INTERSECTION);
}