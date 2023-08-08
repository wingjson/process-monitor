use std::{env, fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct TaskConfig {
    pub name: String,
    // pub trigger_time: String,
    // pub pid: f64,
    pub port: Option<String>,
    pub process:Option<String>,
    pub cmd:Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub tasks: Vec<TaskConfig>,
}


pub fn read_config() ->Result<Config, Box<dyn std::error::Error>>{
    // get path
    if let Ok(current_dir) = env::current_dir() {
        println!("path :{:?}", current_dir);
        let root_dir = current_dir.to_string_lossy().replace(r#"\"#,"/");
        let config_file_path = root_dir + "/config.json";
        println!("path :{:?}",config_file_path);
        let mut file = File::open(config_file_path).expect("Failed to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read config file");
        let config: Config = serde_json::from_str(&contents).expect("Failed to parse config file");
        return Ok(config)
    } else {
        return Err("Failed to get current directory".into());
    }
    
}