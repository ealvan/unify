use crate::utils::toolbox::{get_files,remove_prefix_in_place, store_files};
use crate::actions::operations::Paths;
pub mod utils;
pub mod actions;
use std::path::Path;

const ROOT1: &str = "r1";//"/mnt/e/box_2024-02-23/data/documentsEdsel/COMEDOR"; //E:\box_2024-02-23\data\documentsEdsel\COMEDOR
const ROOT2: &str = "r2";//"/mnt/e/Boxalmost2023-76.8/box/documentsEdsel/COMEDOR"; //E:\Boxalmost2023-76.8\box\documentsEdsel\COMEDOR
const ROOT3: &str = "r3";//"/mnt/e/test_script"; 

const INTERSECTION:&str = "root1_and_root2.log";
const R1_R2:&str = "root1-root2.log";
const R2_R1:&str = "root2-root1.log";

fn main(){
    println!("Unify");
    
    let paths = Paths{
        intersection: String::from(INTERSECTION),
        r1_r2: String::from(R1_R2),
        r2_r1: String::from(R2_R1)
    };

    let root1 = Path::new(&ROOT1);    
    let root2 = Path::new(&ROOT2);
    let root3 = Path::new(&ROOT3);
    
    let mut fr1: Vec<String> = Vec::new();
    let mut fr2: Vec<String> = Vec::new();

    match get_files(&root1, &mut fr1){
        Ok(_) => println!("Succesfully get root1's files"),
        Err(why) => panic!("The error is: {}", why)
    }
    match get_files(&root2, &mut fr2){
        Ok(_) => println!("Succesfully get root2's files"),
        Err(why) => panic!("The error is: {}", why)
    }
    let mut prefix = root1.display().to_string();
    remove_prefix_in_place(&mut fr1, &prefix);
    prefix = root2.display().to_string();
    remove_prefix_in_place(&mut fr2, &prefix);
    store_files(fr1, fr2, paths);

}