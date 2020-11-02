use crypto_hash::{Algorithm, hex_digest};

pub fn run() {
    println!("{}", crack_hash("c1ec8dd44a4f9c19fe8c7ae9b5592d24".to_string()).unwrap());
}

fn crack_hash(string: String) -> Result<i32, ()> {
    let mut counter: i32 = 0;
    loop {
        let formatted_string = format!("{:0>5}", counter.to_string());
        if string == hex_digest(Algorithm::MD5, formatted_string.as_bytes()) {
            return Ok(counter);
        }
        if counter == 99999{
            return Err(());
        }
        counter += 1;
    }
}