use std::fs;
use std::path::{Path, PathBuf};

fn main()
{
    let testpath = "./TestData/ScanTest";
    let mut files = Vec::new();
    scan_directory(Path::new(testpath), &mut files);
    for file in &mut files
    {
        println!("{}", file.display());
    }
}

fn scan_directory(path: &Path, files: &mut Vec<PathBuf>)
{
    let entries = fs::read_dir(path).unwrap(); // returns Result<ReadDir, Error> -> unwrap returns ReadDir

    for entry in entries // entry is of type Result<DirEntry, Error>
    {
        let entry = entry.unwrap();
        let entrypath = entry.path(); //returns a PathBuf for each file/directory

        if entry.file_type().unwrap().is_dir()
        {
            scan_directory(&entrypath, files); //returns a Vec<PathBuf>
        } else {
            files.push(entrypath);
        }
    }
}
