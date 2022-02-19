use colored::Colorize;
use ipnet::Ipv4Net;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
extern crate ipnet;
fn main() {
    
    let args:Vec<String> = env::args().collect();
    let mode = &args[1];
    let filename = &args[2];
    let mut ips_string:Vec<String> = vec![];
    let mut iprange_string:Vec<String> = vec![];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if(mode == "c"){
                    if let Ok(temp) = check_c(&ip){
                        if !iprange_string.contains(&temp){
                            iprange_string.push(temp);
                        }
                        
                        if !ips_string.contains(&ip){
                            ips_string.push(ip);
                        }
                    }else{
                        println!("there is a error at \"{}\"",ip.to_string().red());
                        continue;
                    }
                    
                }else if (mode =="b") {
                    if let Ok(temp) = check_b(&ip){
                        if !iprange_string.contains(&temp){
                            iprange_string.push(temp);
                        }
                        
                        if !ips_string.contains(&ip){
                            ips_string.push(ip);
                        }
                    }else{
                        println!("there is a error at \"{}\"",ip.to_string().red());
                        continue;
                    }
                }
                
            }  
        }   
    }

    print_result(iprange_string, ips_string);
    

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_c(temp:&String)->Result<String,String>{
    let temp:Vec<&str> = temp.split(".").collect();
    if temp.len()==4{
        Ok(temp[0].to_string()+"."+temp[1]+"."+temp[2]+".0/24")
    }else {
        Err("read ip error!".to_string())
    }
}

fn check_b(temp:&String)->Result<String,String>{
    let temp:Vec<&str> = temp.split(".").collect();
    if temp.len()==4{
        Ok(temp[0].to_string()+"."+temp[1]+".0.0/16")
    }else {
        Err("read ip error!".to_string())
    }
    
}

fn print_result(iprange_string:Vec<String>,ips_string:Vec<String>){
    let ip_range: Vec<Ipv4Net> = iprange_string.iter().map(|s| s.parse().unwrap()).collect();
    let ip_s:Vec<Ipv4Addr> = ips_string.iter().map(|s| s.parse().unwrap()).collect();
    let mut iphash:HashMap<Ipv4Net,Vec<Ipv4Addr>> = HashMap::new();
    for i in ip_range.iter(){
        let mut temp:Vec<Ipv4Addr> = vec![];
        for j in &ip_s{
            if i.contains(j){
                //println!("{}=>{}",i,j);
                temp.push(j.clone());
            }
        }
        iphash.insert(i.clone(), temp.clone());
    }
    for i in iphash{
        print!("{}=>",i.0.to_string().green());
        for j in i.1{
            print!("  {}",j.to_string().yellow());
        }
        println!("");
    }

}
