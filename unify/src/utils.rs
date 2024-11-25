pub mod toolbox{
    use std::io::{self, Write};
    use std::fs::{self, OpenOptions, DirBuilder};
    use std::path::Path;
    use std::collections::HashSet;
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
}

