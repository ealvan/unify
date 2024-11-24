pub mod toolbox{
    use std::io::{self, Write};
    use std::fs::{self, OpenOptions, DirBuilder};
    use std::path::Path;
    use std::collections::HashSet;
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
    pub fn write_rootn_files(rootn:&HashSet<String>, root_log_name: &str) -> std::io::Result<()>{
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
    fn create_dir_path(path:&str){
        DirBuilder::new().recursive(true).create(path).unwrap();
    }
}

