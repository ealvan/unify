#[warn(unused_variables)]
use std::io;
use std::fs::{self, OpenOptions};
use std::fs::File;
// use rayon::prelude::*;
use log::{info, error};
use fern::Dispatch;
use std::collections::HashSet;
use std::path::Path;
use std::io::Write;

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
fn write_rootn_files(rootn:&Vec<String>, root_log_name: &str) -> std::io::Result<()>{
    let mut content: String = rootn.join("\n");
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(root_log_name)
        .expect("error opening a file");

    f.write_all(content.as_bytes()).expect("Unable to write");
    Ok(())
}
fn main(){
    println!("FIRST PHASE");
    //define root1, root2 & root3
    let root1 = Path::new("test/r1");    
    let root2 = Path::new("test/r2");
    let root3 = Path::new("/test/r3");    
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
    let root1_logname = "root1_file.log";
    let root2_logname = "root2_file.log";
    write_rootn_files(&fr1, &root1_logname);
    write_rootn_files(&fr2, &root2_logname);

}