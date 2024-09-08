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
use std::fs::File;

use std::path::Path;
use std::collections::HashSet;

fn get_files(dir: &Path, v: &mut HashSet<String>) -> io::Result<()> {
    if dir.is_dir(){
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                match get_files(&path, v) {
                    Ok(_) => println!("Succesfully worked!"),
                    Err(why) => println!("The error is: {}", why),
                };
            }else {
                // println!("{}", path.display());
                v.insert(String::from(path.to_str().unwrap()));                
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
fn remove_prefix_in_place(set: &mut HashSet<String>, prefix: &str){
    let new_set : HashSet<String> = set.iter().map(|entry| {
        if entry.starts_with(prefix) {
            entry.strip_prefix(prefix).unwrap_or(entry).to_string()
        } else{
            entry.to_string()
        }
    }).collect();
    
    set.clear();
    
    set.extend(new_set);

}

fn get_filesize(path: &String, prefix: &str) -> std::io::Result<u64>{
    let complete_path = prefix.to_string() + path;
    let fpath = Path::new(&complete_path);
    let f = File::open(fpath)?;
    let metadata = f.metadata()?;
    // println!("{:#?}", metadata);
    Ok(metadata.len())    
}

// fn get_complete_path(prefix:&str, file:&str) -> String{
//     String::from(prefix + file)
// }

fn do_versions(common_files: &HashSet<String>, prefix_r1: &str, prefix_r2: &str)  {
    for file in common_files {
        let file_size_r1 = match get_filesize(&file, &prefix_r1){
            Err(why) => panic!("Error why: {}", why),
            Ok(file_size) => file_size,
        };
        let file_size_r2 = match get_filesize(&file, &prefix_r2){
            Err(why) => panic!("Error why: {}", why),
            Ok(file_size) => file_size, 
        };
        
        if file_size_r1 == file_size_r2{
            println!("Are the same");
            //move the file to root3
            println!("Movinf the file .........");
            
        }else{
            println!("You need versioning!");
            //change their names to -v1 -v2 and move them
        }

        // println!("file_r1 size = {}, file_r2 size= {}",file_size_r1, file_size_r2 );
    }

    // Ok(())
}
fn move_file() {

}


fn main() -> io::Result<()> {
    let mut r1: HashSet<String> = HashSet::new();
    let mut r2: HashSet<String> = HashSet::new();

    let rpath_r1 = Path::new("../test/r1");
    let rpath_r2 = Path::new("../test/r2");

    match get_files(&rpath_r1, &mut r1) {
        Ok(_) => println!("Succesfully worked!"),
        Err(why) => println!("The error is: {}", why),
    };
    match get_files(&rpath_r2, &mut r2) {
        Ok(_) => println!("Succesfully worked!"),
        Err(why) => println!("The error is: {}", why),
    };
    let prefix_r1 = "../test/r1";
    let prefix_r2 = "../test/r2";

    // println!("R1");
    // for f in &r1{
    //     println!("{}",f);
    // }
    // println!("R2");
    // for f in &r2{
    //     println!("{}",f);
    // }
    remove_prefix_in_place(&mut r1, &prefix_r1);
    remove_prefix_in_place(&mut r2, &prefix_r2);
    println!("R1");
    for f in &r1{
        println!("{}",f);
    }
    println!("R2");
    for f in &r2{
        println!("{}",f);
    }

    let common: HashSet<_> = r1.intersection(&r2).cloned().collect();
    println!("Common files are: {:?}", common);
    let dif1: HashSet<_> = r1.difference(&r2).cloned().collect();
    println!("r1 - r2: {:?}", dif1);
    let dif2: HashSet<_> = r2.difference(&r1).cloned().collect();
    println!("r2 - r1: {:?}", dif2);

    do_versions(&common, &prefix_r1, &prefix_r2);

    Ok(())
}





