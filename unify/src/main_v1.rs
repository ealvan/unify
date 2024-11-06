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
use std::fs;
use std::fs::File;
// use rayon::prelude::*;
use log::{info, error};
use fern::Dispatch;
use std::collections::HashSet;
use std::path::{Path};
// use std::io::Write;
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
            Err(why) => {
                error!("Error in get_filesize() : {}, data: {}{}", why, prefix_r1,file);
                0
            },
            Ok(file_size) => file_size,
        };

        let file_size_r2 = match get_filesize(&file, &prefix_r2){
            Err(why) => {
                error!("Error in get_filesize() : {}, data: {}{}", why, prefix_r2,file);
                0
            },
            Ok(file_size) => file_size,
        };
        let complete_path1 = prefix_r1.to_string() + file;
        let complete_path2 = prefix_r2.to_string() + file;
        if file_size_r1 != 0 && file_size_r2 != 0 {
            if file_size_r1 == file_size_r2{
                // println!("Are the same");
                //move the file to root3
                info!("EQUAL - Moving the file: \nfrom: {} size: {} -- to:{} size: {}",complete_path1,file_size_r1,complete_path2,file_size_r2 );
                let parent = Path::new(&move_path).parent().unwrap();//not basename
                
                match fs::create_dir_all(&parent){
                    Ok(_) => (),
                    Err(why) => error!("Error in create path why: {}, data: {}", why, parent.display()),
                }
                match &Path::new(&move_path).try_exists() {
                    Ok(val) => {
                        if *val == false {
                            match fs::rename(&complete_path, &move_path){
                                Ok(_) => (),
                                Err(why) => error!("Error why: {}, data: complete_path: {}, move_path: {}", why, complete_path, move_path),
                            };    
                        }
            
                    },
                    Err(why) => error!("move path undetermined : {} , data: move_path: {}", why, move_path),
                }
                // let mut f = File::open(complete_path);
                
            }else{
                info!("DIFFERENT -  Moving the file: \nfrom: {} size: {} -- to:{} size: {}",complete_path1,file_size_r1,complete_path2,file_size_r2 );

            }
            // println!("file_r1 size = {}, file_r2 size= {}",file_size_r1, file_size_r2 );
        }
        if file_size_r1 != 0 || file_size_r2 != 0{
            // info!("Versioning file: {}", complete_path);
            let pr1 = prefix_r1.to_string() + &file;
            let pr2 = prefix_r2.to_string() + &file;
            let pfr1 = Path::new(&pr1);
            let pfr2 = Path::new(&pr2);

            let new_path = prefix_r3.display().to_string() + &file;
            let new_path = Path::new(&new_path).parent().unwrap(); 

            let filename = pfr1.file_name().unwrap();
            let f_v1 = new_path.join("v1-".to_owned()+filename.to_str().unwrap());
            let f_v2 = new_path.join("v2-".to_owned()+filename.to_str().unwrap());
            // info!("version -> {}, {}", f_v1.display(), f_v2.display());
            
            // match fs::create_dir_all(&new_path){
            //     Ok(_) => info!("Creating path : {}", &new_path.display()),//for path
            //     Err(why) => error!("Error why: {}, new_path: {}", why, new_path.display()),
            // }
            // if file_size_r1 != 0{
            //     match fs::rename(&pfr1, &f_v1){
            //         Ok(_) => info!("Version 1 - f_v1: {}", f_v1.display()),
            //         Err(why) => error!("Error why: {}, data: pfr1: {}, f_v1: {}", why, pfr1.display(), f_v1.display()),
            //     };    
            // }
            // if file_size_r2 != 0{
            //     match fs::rename(&pfr2, &f_v2){
            //         Ok(_) => info!("Version 2 - f_v2: {}", f_v2.display()),
            //         Err(why) => error!("Error why: {}, data: pfr2: {}, f_v2: {}", why, pfr2.display(), f_v2.display()),
            //     };    
            // }

        }
        complete_path = complete_path.strip_suffix(&*file).expect("There is no that prefix").to_string();
        move_path = move_path.strip_suffix(&*file).expect("There is no that prefix").to_string();

    }
}
fn move_file(dif_set: &HashSet<String>, prefix_from: &String, prefix_to: &Path) {
    for file in dif_set{
        let filesize = match get_filesize(&file, &prefix_from){
            Err(why) => {
                error!("Error in get_filesize() : {}, data: {}{}", why, prefix_from,file);
                0
            },
            Ok(file_size) => file_size,
        };
        if filesize != 0 {
            let from = prefix_from.to_string() + file;
            let to = prefix_to.display().to_string()+file;
            // let pfrom = Path::new(&from);
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
            
            match fs::rename(&from, &to){
                Ok(_) => info!("OK - Move operation from:{} to:{}", &from, &to),
                Err(why) => error!("Move Error : {}, from: {}, to: {}", why, &from, &to),
            }
        }
    }
}
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create or open the log file
    let file = File::create("file.log")?;

    // Set up the logging format
    let file_dispatcher = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(file);

    let stdout_dispatcher = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    // Apply the log configuration
    fern::Dispatch::new()
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;

    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let mut r1: Vec<String> = Vec::new();
    let mut r2: Vec<String> = Vec::new();

    let rpath_r1 = Path::new("/mnt/e/boxHandMade");//../test/r1
    let rpath_r2 = Path::new("/mnt/e/Boxalmost2023-76.8");//../test/r2
    let rpath_r3 = Path::new("/mnt/e/merge_box");
    println!("\nGetting ROOT1: {}", rpath_r1.display());
    match get_files(&rpath_r1, &mut r1) {
        Ok(_) => info!("getfiles from root1: {}",rpath_r1.display()),//println!("Succesfully worked!")
        Err(why) => error!("The error is: {}, data: {}", why, rpath_r1.display()),
    };
    println!("\nGetting ROOT2: {}", rpath_r2.display());
    match get_files(&rpath_r2, &mut r2) {
        Ok(_) => info!("getfiles from root1: {}",rpath_r2.display()),//println!("Succesfully worked!")
        Err(why) => error!("The error is: {}, data: {}", why, rpath_r2.display()),
    };
    let prefix_r1 = rpath_r1.display().to_string();
    let prefix_r2 = rpath_r2.display().to_string();

    println!("\nRemoving prefix ROOT1: {}", prefix_r1);
    remove_prefix_in_place(&mut r1, &prefix_r1);

    println!("\nRemoving prefix ROOT2: {}", prefix_r2);
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
    println!("\nintersection r1 & r2");
    let common: HashSet<_> = r1_set.intersection(&r2_set).cloned().collect();
    // println!("Common files are: {:#?}", common);
    println!("\ndifference r1 - r2");
    let dif1: HashSet<_> = r1_set.difference(&r2_set).cloned().collect();
    // println!("r1_set - r2_set: {:#?}", dif1);
    println!("\ndifference r2 - r1");
    let dif2: HashSet<_> = r2_set.difference(&r1_set).cloned().collect();
    // println!("r2 - r1_set: {:#?}", dif2);

    println!("\nDO VERSIONS");

    do_versions(&common, &prefix_r1, &prefix_r2, &rpath_r3);
    println!("\nDO MOVE FILES root1");
    // move_file(&dif1, &prefix_r1, &rpath_r3);
    println!("\nDO MOVE FILES root2");
    // move_file(&dif2, &prefix_r2, &rpath_r3);

    println!("\nFIN");
    Ok(())
}





