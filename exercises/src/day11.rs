use std::io;

pub fn run(){
    let mut array: Vec<Vec<i32>> = Vec::with_capacity(6 * 6);
    for i in 0..6 {
        array.push(vec![0; 6]);
    }
    // Read lines
    for i in 0..6 {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading");
        parse_string(input.trim().to_string(), &mut array, i);
    }
    let mut max: i32 = -1000;
    for a in 0..4 {
        for i in 0..4 {
            let current: i32;
            current = array[i][a] + array[i][a+1] + array[i][a+2] + array[i+1][a+1] + array[i+2][a] + 
                array[i+2][a+1] + array[i+2][a+2];
            if current > max {
                max = current;
            }
        }
    }
    
    println!("{}", max);
}

fn parse_string(string: String, array: &mut Vec<Vec<i32>>, row: i32) {
    let mut column: i32 = 0;
    let mut i = 0;
    loop {
        if string.as_bytes()[i] != ' ' as u8 {
            if string.as_bytes()[i] != '-' as u8 {
                array[row as usize][column as usize] = string.as_bytes()[i] as i32 - '0' as i32;
                column += 1;
            }else{
                array[row as usize][column as usize] = (string.as_bytes()[i+1] as i32 - '0' as i32) * (-1);
                column += 1;
                i += 1;
            }
        }
        i += 1;
        if i == string.len() {
            break;
        }
    }
}
