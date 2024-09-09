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
// use rayon::prelude::*;

use std::collections::HashSet;
use std::path::Path;

fn get_files(dir: &Path, v: &mut Vec<String>) -> io::Result<()> {

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            match get_files(&path, v) {
                Ok(_) => print!("."),//println!("Succesfully worked!")
                Err(why) => println!("The error is: {}", why),
            };
        }else {
            // println!("{}", path.display());
            v.push(String::from(path.to_str().unwrap()));                
        }
    }
    Ok(())
}
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

fn remove_prefix_in_place(myvec: &mut Vec<String>, prefix: &str){
    *myvec = myvec.iter()
              .filter_map(|entry| entry.strip_prefix(prefix).map(|e| e.to_string()))
              .collect();
    // let new_set : HashSet<String> = set.iter().map(|entry| {
    //     if entry.starts_with(prefix) {
    //         entry.strip_prefix(prefix).unwrap_or(entry).to_string()
    //     } else{
    //         entry.to_string()
    //     }
    // }).collect();
    
    // set.clear();
    
    // set.extend(new_set);

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

fn do_versions(common_files: &HashSet<String>, prefix_r1: &str, prefix_r2: &str, prefix_r3: &Path)  {
    let mut complete_path = prefix_r1.to_string();

    let mut move_path = prefix_r3.display().to_string();
    for file in common_files {
        complete_path += file;
        move_path += file;
        let file_size_r1 = match get_filesize(&file, &prefix_r1){
            Err(why) => panic!("Error why: {}", why),
            Ok(file_size) => file_size,
        };

        let file_size_r2 = match get_filesize(&file, &prefix_r2){
            Err(why) => panic!("Error why: {}", why),
            Ok(file_size) => file_size, 
        };
        
        if file_size_r1 == file_size_r2{
            // println!("Are the same");
            //move the file to root3
            println!("Moving the file: \nfrom: {} -- to:{}",complete_path,move_path );
            let parent = Path::new(&move_path).parent().unwrap();//not basename
            
            match fs::create_dir_all(&parent){
                Ok(_) => println!("f"),
                Err(why) => println!("Error why: {}", why),
            }
            match &Path::new(&move_path).try_exists() {
                Ok(val) => {
                    if *val == false {
                        match fs::rename(&complete_path, &move_path){
                            Ok(_) => println!("b"),
                            Err(why) => panic!("Error why: {}", why),
                        };    
                    }
        
                },
                Err(why) => println!("move path undetermined : {}", why),
            }
            // let mut f = File::open(complete_path);
            
        }else{
            println!("Versioning file: {}", complete_path);
            let from_path_r1 = Path::new(&(prefix_r1.to_string() + file));
            let from_path_r2 = Path::new(&(prefix_r2.to_string() + file));
            let f_v1 = prefix_r3.display().to_string() + "v1"+file;
            let f_v2 = prefix_r3.display().to_string() + "v2"+file;
            println!("version -> {}, {}", f_v1, f_v2);
            // match fs::rename(&from_path_r1, &move_path){
            //     Ok(_) => println!("b"),
            //     Err(why) => panic!("Error why: {}", why),
            // };
            // match fs::rename(&from_path_r2, &move_path){
            //     Ok(_) => println!("b"),
            //     Err(why) => panic!("Error why: {}", why),
            // };
            
            //change their names to -v1 -v2 and move them
        }
        complete_path = complete_path.strip_suffix(&*file).expect("There is no that prefix").to_string();
        move_path = move_path.strip_suffix(&*file).expect("There is no that prefix").to_string();

        // println!("file_r1 size = {}, file_r2 size= {}",file_size_r1, file_size_r2 );
    }

    // Ok(())
}
fn move_file(dif_set: &HashSet<String>, prefix_from: &String, prefix_to: &Path) {
    for file in dif_set{
        let from = prefix_from.to_string() + file;
        let to = prefix_to.display().to_string()+file;
        let pfrom = Path::new(&from);
        let pto = Path::new(&to);
        let parent = pto.parent().unwrap();

        match fs::create_dir_all(&parent){
            Ok(_) => print!("-"),//for path
            Err(why) => println!("Error why: {}", why),
        }

        // match &pfrom.try_exists(){
        //     Ok(_) => (),
        //     Err(why) => panic!("FROM Error : {}",why),
        // }
        // match &pto.try_exists(){
        //     Ok(_) => (),
        //     Err(why) => panic!("TO Error : {}",why),
        // }
        
        match fs::rename(from, to){
            Ok(_) => print!("."),
            Err(why) => panic!("Move Error : {}", why),
        }
        
    }
}

fn main() -> io::Result<()> {
    let mut r1: Vec<String> = Vec::new();
    let mut r2: Vec<String> = Vec::new();

    let rpath_r1 = Path::new("/mnt/e/boxHandMade/models");
    let rpath_r2 = Path::new("/mnt/e/Boxalmost2023-76.8/models");
    let rpath_r3 = Path::new("/mnt/e/box_merge");
    match get_files(&rpath_r1, &mut r1) {
        Ok(_) => print!("."),//println!("Succesfully worked!")
        Err(why) => println!("The error is: {}", why),
    };
    match get_files(&rpath_r2, &mut r2) {
        Ok(_) => print!("."),//println!("Succesfully worked!")
        Err(why) => println!("The error is: {}", why),
    };
    let prefix_r1 = rpath_r1.display().to_string();
    let prefix_r2 = rpath_r2.display().to_string();

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
    // println!("R1");
    // for f in &r1{
    //     println!("{}",f);
    // }
    // println!("R2");
    // for f in &r2{
    //     println!("{}",f);
    // }
    let r1_set: HashSet<_> = r1.into_iter().collect();
    let r2_set: HashSet<_> = r2.into_iter().collect();

    let common: HashSet<_> = r1_set.intersection(&r2_set).cloned().collect();
    // println!("Common files are: {:#?}", common);
    let dif1: HashSet<_> = r1_set.difference(&r2_set).cloned().collect();
    // println!("r1_set - r2_set: {:#?}", dif1);
    let dif2: HashSet<_> = r2_set.difference(&r1_set).cloned().collect();
    // println!("r2 - r1_set: {:#?}", dif2);

    do_versions(&common, &prefix_r1, &prefix_r2, &rpath_r3);
    move_file(&dif1, &prefix_r1, &rpath_r3);
    move_file(&dif2, &prefix_r2, &rpath_r3);
    println!("");
    Ok(())
}





