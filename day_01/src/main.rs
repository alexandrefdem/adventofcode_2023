use std::{fs::read_to_string};
use std::collections::HashMap;

use regex::Regex;

fn check_matches_replace_on_string(s: &String, text_hashmap: &HashMap<String,String>)->String{
    println!("{}", s);
    let mut new_str = s.to_string();
    let mut start_index:usize = 0;
    let mut end_index:usize = start_index+2;
    let mut maxsize = s.len();
    
    while start_index+2<maxsize{
        if end_index<=maxsize{
            println!("{}", &new_str[start_index..end_index]);
            if text_hashmap.contains_key(&new_str[start_index..end_index]){
                new_str = new_str.replace(&new_str[start_index..end_index],text_hashmap.get(&new_str[start_index..end_index]).unwrap());
                maxsize = new_str.len();
                end_index=start_index+1;
            }
        }
        end_index+=1;
        if end_index-start_index>5{
            start_index+=1;
            end_index=start_index+2;
        }
    }
    println!("{}", new_str);
    new_str
}

fn main() {
    let re = Regex::new(r"[a-zA-Z]").unwrap();

    let mut text_to_num: HashMap<String, String> = HashMap::new();
    text_to_num.insert("one".to_string(), "o1e".to_string());
    text_to_num.insert("two".to_string(), "t2o".to_string());
    text_to_num.insert("three".to_string(), "t3e".to_string());
    text_to_num.insert("four".to_string(), "f4r".to_string());
    text_to_num.insert("five".to_string(), "f5e".to_string());
    text_to_num.insert("six".to_string(),"s6x".to_string());
    text_to_num.insert("seven".to_string(), "s7n".to_string());
    text_to_num.insert("eight".to_string(), "e8t".to_string());
    text_to_num.insert("nine".to_string(), "n9e".to_string());


    let pattern = read_to_string("./src/input.txt").unwrap()
    .split("\n")
    .map(|x| -> String {re.replace_all(check_matches_replace_on_string(&x.to_string(),&text_to_num).as_str(),"").to_string()})
    .collect::<Vec<_>>();

    let result = pattern.iter().map(|x| -> u32{
        if x.len()==1{
            let mut cloned = x.clone();
            cloned.push_str(x);
            cloned.parse::<u32>().unwrap()
        }
        else if x.len()>2{
            let mut return_str = String::new();

            return_str.push(x.chars().next().unwrap());
            return_str.push(x.chars().last().unwrap());
            return_str.parse::<u32>().unwrap()
        }
        else{
            x.parse::<u32>().unwrap()
        }
    }).collect::<Vec<u32>>().into_iter().reduce(|acc, input| acc + input).unwrap();

    println!("Result is: {}", result);
}
