
use std::{net::TcpStream, time::Duration};

use sysinfo::{System, SystemExt};

pub struct ProcessInfo{
    sys: System,
}

impl ProcessInfo {
    /**
     * @description: init system
     * @return {*}
     */    
    pub fn new()  -> ProcessInfo{
        let mut sys = System::new_all();
        sys.refresh_all();
        ProcessInfo { sys }
    }

    /**
     * @description: check by name
     * @param {*} self
     * @return {*}
     */    
    pub fn check_by_name(&self,name:&String)->bool{
        let mut process_iter = self.sys.processes_by_name(&name);
        // if don't have next,it means the process was exited
        if let Some(_process) = process_iter.next() {
            return true;
        } else {
            return false;
        }
    }

    /**
     * @description: check by port
     * @param {*} self
     * @return {*}
     */    
    pub fn check_by_port(&self,port:i32)->bool{
        let addr = format!("127.0.0.1:{}", port);
        // if connect is connected,the process is running
        match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(1)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /**
     * @description: get os name
     * @param {*} self
     * @return {*}
     */    
    pub fn get_os_info(&self)->String{
        let os = std::env::consts::OS;
        println!("{}",os);
        return os.to_owned();
    }

}