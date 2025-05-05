pub mod toolbox{
    use std::io::{self, Write};
    use std::fs::{self, OpenOptions, DirBuilder, read_to_string};
    use std::path::Path;
    use std::collections::HashSet;
    use std::time::SystemTime;
    

    use crate::actions::operations::Paths;

    pub fn get_files(dir: &Path, v:&mut Vec<String>) -> io::Result<()> {
        for entry in fs::read_dir(dir)?{
            let mentry = match entry{
                Ok(dir_entry) => dir_entry,
                Err(why) => panic!("Error: {why}")
            };

            let path = mentry.path();
            if path.is_dir(){
                get_files(&path, v)?;
            }else{
                v.push(String::from(path.to_str().unwrap()));
            }
        }
        Ok(())
    }
    pub fn print_vector(rootn: &Vec<String>){
        for f in rootn{
            println!("{}",f);
        }
    }
    pub fn remove_prefix_in_place(myvec: &mut Vec<String>, prefix: &str){
        *myvec = myvec.iter()
                  .filter_map(|entry| entry.strip_prefix(prefix).map(|e| e.to_string()))
                  .collect();
    }
    pub fn create_dir(dir_path: &str) -> Result<(),io::Error>{
        fs::create_dir(dir_path)?;
        Ok(())
    }
    pub fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
        fs::rename(source, destination)?; // Move the file from source to destination
        println!("source: {} -- destination: {}", source, destination);
        Ok(()) // Return Ok if the operation completes successfully
    }
    pub fn delete_file(file_path: &str) -> Result<(), io::Error> {
    
        fs::remove_file(file_path)?; // Delete the file at the specified path
        println!("Remove file: {} ... OK", file_path);
        Ok(()) // Return Ok if the operation completes successfully
    }
    pub fn create_dir_path(path:&str){
        DirBuilder::new().recursive(true).create(path).unwrap();
    }
    pub fn write_rootn_files(rootn:&HashSet<String>, root_log_name: &str) -> std::io::Result<()>{
        let content: String = itertools::join(rootn, "\n");
        println!("{}",content);
        let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(root_log_name)
        .expect("error opening a file");

        f.write_all(content.as_bytes()).expect("Unable to write");
        Ok(())
    }
    pub fn store_files(fr1: Vec<String>, fr2: Vec<String>, paths: Paths){
        let root1_set: HashSet<_> = fr1.into_iter().collect();
        let root2_set: HashSet<_> = fr2.into_iter().collect();
        
        let INTERSECTION = paths.intersection;
        let R1_R2 = paths.r1_r2;
        let R2_R1 = paths.r2_r1;

        let common: HashSet<String> = root1_set.intersection(&root2_set).cloned().collect();
        write_rootn_files(&common, &INTERSECTION);
        let dif1: HashSet<String> = root1_set.difference(&root2_set).cloned().collect();
        write_rootn_files(&dif1, &R1_R2);
        let dif2: HashSet<String> = root2_set.difference(&root1_set).cloned().collect();
        write_rootn_files(&dif2, &R2_R1);
    }
    pub fn move_files(from: &str, to: &str, prefix: &str){
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
    fn get_filesize_and_lastupdate(file_path: &str) -> Result<(u64,SystemTime), io::Error>{
        let metadata = fs::metadata(&file_path)?;
        let time = metadata.created()?;
        Ok((metadata.len(), time))
    }
    
    pub fn do_versions(intersection_file: &str, ROOT1: &str, ROOT2: &str, ROOT3: &str){
        for line in read_to_string(intersection_file).unwrap().lines(){
            let mut root1_file = String::from(ROOT1);
            let mut root2_file = String::from(ROOT2);
            let mut root3_file = String::from(ROOT3);
            let now = SystemTime::now();
            root1_file.push_str(&line);
            root2_file.push_str(&line);
            let r1_exists: &Path = Path::new(&root1_file);
            let r2_exists: &Path = Path::new(&root2_file);
            let r1_exists = r1_exists.exists();
            let r2_exists = r2_exists.exists();
            print!("f1: {}", root1_file);
            if r1_exists & r2_exists{
                println!("f2: {}", root2_file);
                let (fr1, lacc1) = match get_filesize_and_lastupdate(&root1_file) {
                    Ok(val) => (val.0, val.1),
                    Err(why) => {
                        println!("error: {}",why);
                        (1000, now)
                    }
                };
                let (fr2, lacc2) = match get_filesize_and_lastupdate(&root2_file) {
                    Ok(val) => (val.0, val.1),
                    Err(why) => {
                        println!("error: {}",why);
                        (1000, now)
                    }
                };
                root3_file.push_str(&line);
                let path_version = Path::new(&root3_file);
                let parent_path = path_version.parent().unwrap();
                let parent_exists = match path_version.parent() {
                    Some(p) => p.exists(),
                    None => false
                };
                // let parent_path_ = Path::new(&parent_path);
                if ! parent_exists {
                    let pdest = parent_path.display().to_string();
                    create_dir_path(&pdest);
                }
                if fr1 == fr2{
                    // delete_file(&)
                    println!("== > File r1: {}\nFile r2: {} size: {}", &root1_file,&root2_file, &fr1);
                    println!("lacc1: {:?} lacc2: {:?}",lacc1, lacc2);
                    let isnewer = match lacc1.duration_since(lacc2) {
                        Ok(duration) => {
                            println!("File1 is {} seconds newer than File2.", duration.as_secs());
                            true
                        },
                        Err(e) => {
                            println!("File2 is newer by {} seconds.", e.duration().as_secs());
                            false
                        },
                    };
                    if isnewer {
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
                    let parent_path_filename = parent_path.to_str().unwrap(); 
                    let new_version_path = format!("{parent_path_filename}/v1_{filename}");
                    
                    match move_file(&root1_file, &new_version_path) {
                        Ok(_) => println!("move_file OK"),
                        Err(why) => println!("{}",why)
                    };
                    let new_version_path = format!("{parent_path_filename}/v2_{filename}");
    
                    match move_file(&root2_file, &new_version_path) {
                        Ok(_) => println!("move_file OK"),
                        Err(why) => println!("{}",why)
                    };
                }
    
            }
        }
    }
    
    
}