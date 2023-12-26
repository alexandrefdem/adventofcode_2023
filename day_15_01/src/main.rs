use std::fs::read_to_string;

fn hash_algorithm(string_input:&str)->u32{
    //Determine the ASCII code for the current character of the string.
    //Increase the current value by the ASCII code you just determined.
    //Set the current value to itself multiplied by 17.
    //Set the current value to the remainder of dividing itself by 256.
    let mut current_value:u32=0;
    for c in string_input.chars().into_iter(){
        let ascii_code: u32 = c.clone() as u32;
        current_value = current_value + ascii_code;
        current_value = current_value*17;
        current_value = current_value%256;
    }
    current_value
}

fn main() {
    let mut file_input:Vec<String> = Vec::new();

    for s in read_to_string("./src/input.txt").unwrap().split(',').into_iter(){
        file_input.push(s.to_string());
    }

    let mut result:u32=0;

    for item in file_input.iter(){
        let hash_return = hash_algorithm(item);
        result+=hash_return;
    }
    println!{"Result is {:?}", result};
}
