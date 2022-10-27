use std::{path::PathBuf, str::FromStr, error::Error, env, fs};


#[derive(Debug)]
pub struct LanguageExample {
    language: Option<String>,
    dir: String,
    start_point: String, // What is the start point of an application in the language, like in python if __name__ == "__main__":
    dep_filename: String, // name of the dependency file / files
    execute_command: String,
    install_deps_command: String
}
impl LanguageExample {
    pub fn new( dir: String, start_point: String, dep_filename: String, execute_command: String, install_deps_command: String) -> Self {
        return LanguageExample {
            language: None,
            dir,
            start_point,
            dep_filename,
            execute_command,
            install_deps_command
        }
    }
    fn get_files(&self, path: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>>{
        let mut file_paths = vec![];
        let paths = fs::read_dir(path.clone())?;
        for x in paths {
            let value = x?.path();
            if value.is_dir(){
                let mut res = self.get_files(value);
                file_paths.append(&mut res?);
            }else{
                file_paths.push(value);
            }
        }
        Ok(file_paths)
    }

    pub fn filter_executable(&self) -> Result<Vec<PathBuf>, Box<dyn Error>>{
        let path = PathBuf::from_str(self.dir.as_str())?;
        let files = self.get_files(path)?;
        
        let mut executables = vec![];

        let pattern = self.start_point.as_str();

        for x in files {
            let buf = match fs::read_to_string(&x){
                Ok(buf) => buf,
                Err(err) => continue
            };
            if buf.contains(pattern){
                executables.push(x);
            }
        }
        Ok(executables)
    }

    pub fn get_dependencie_files(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let path = PathBuf::from_str(self.dir.as_str())?;
        let files = self.get_files(path)?;

        let mut deps = vec![];
        
        let pattern = self.dep_filename.as_str();

        for x in files {
            let filename = &x.file_name();
            if filename.is_some(){
                if filename.unwrap().to_str().unwrap().contains(pattern){
                    deps.push(x);
                }
            }
        }

        Ok(deps)
    }

}

#[test]
fn test_get_executable_files() {
    let python = LanguageExample::new(
        String::from("/home/michaell/development/retarus_sdk/retarus-python/examples"),
        String::from("if __name__ == "),
        String::from("requirements.txt"),
        String::from("python3"),
        String::from("pip install -r ")
    );
    let res = python.filter_executable();
    println!("Executable: {:?}", res);
    assert!(false)
}

#[test]
fn test_get_deps() {
    let python = LanguageExample::new(
        String::from("/home/michaell/development/retarus_sdk/retarus-python/examples"),
        String::from("if __name__ == "),
        String::from("requirements.txt"),
        String::from("python3"),
        String::from("pip install -r ")
    );
    let deps = python.get_dependencie_files();
    println!("Deps: {:?}", deps);
    assert!(false)

}