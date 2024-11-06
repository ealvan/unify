#![allow(unused)]
use std::fs;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("/home/debian/projects/unify/algos/algos2")?;
    Ok(())
}
