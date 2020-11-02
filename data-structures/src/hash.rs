use std::process::{Stdio, Command};
use time::Instant;
use std::io::Write;

pub fn run() {
    let start = Instant::now();
    println!("{}", crack_hash("d3eb9a9233e52948740d7eb8c3062d14".to_string()).unwrap());
    println!("{:?}", start.elapsed());
}

fn crack_hash(string: String) -> Result<i32, ()> {
    let mut counter: i32 = 0;
    loop {
        //println!("{}", counter);
        let formatted_string = format!("{:0>5}", counter.to_string());
        let mut cmd2 = Command::new("md5sum")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        
        let stdin = cmd2.stdin.as_mut().unwrap();
        stdin.write_all(formatted_string.as_bytes()).unwrap();
        let output = cmd2.wait_with_output().unwrap();
        let output_str = String::from_utf8(output.stdout).unwrap();
        if string == &output_str[..32] {
            return Ok(counter);
        }
        if counter == 99999{
            return Err(());
        }
        counter += 1;
    }
}