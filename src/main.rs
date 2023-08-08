
mod config;
mod info;
mod win;
mod linux;
use info::ProcessInfo;
use config::{read_config};
use win::{exe_cmd_win};
use linux::{exe_cmd_linux};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    
    if let Ok(config) =  read_config(){
        let info_init = ProcessInfo::new();
        for tasks in config.tasks{
            // only chek port if port exits
           if let Some(port_str) = tasks.port{
                if !port_str.is_empty() {
                    let port = port_str.parse::<i32>().unwrap();
                    let port_res = info_init.check_by_port(port);
                    // port process exit
                    if !port_res {
                        println!("process is not aliving,restarting");
                        if info_init.get_os_info() == "windows" {
                            let _exe_cmd = exe_cmd_win(tasks.cmd.unwrap());
                        }else if  info_init.get_os_info() == "linux"{
                            let _exe_cmd = exe_cmd_linux(tasks.cmd.unwrap());
                        }
                    }
                } else {
                    // no port  use process
                    let process_name = info_init.check_by_name(&tasks.name);
                    // process not alive
                    if !process_name{
                        println!("process is not aliving,restarting");
                        if info_init.get_os_info() == "windows" {
                            let _exe_cmd = exe_cmd_win(tasks.cmd.unwrap());
                        }else if  info_init.get_os_info() == "linux"{
                            let _exe_cmd = exe_cmd_linux(tasks.cmd.unwrap());
                        }
                    }
                }
           }
        }
    }else{
        // ERR
        println!("can not find config file!");
    }
    Ok(())

}
