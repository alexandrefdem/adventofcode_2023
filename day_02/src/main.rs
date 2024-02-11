use std::{fs::read_to_string};
use std::collections::HashMap;

fn check_cubes(cs: &HashMap<String,u16>, red_cubes: &u16, blue_cubes: &u16, green_cubes: &u16)->i16{
    if cs.contains_key("red"){
        if cs.get("red").unwrap() <=red_cubes{
            return 1
        }
        else{
            return -9
        }
    }
    else if cs.contains_key("blue"){
        if cs.get("blue").unwrap() <=blue_cubes{
            return 1
        }
        else{
            return -9
        }
    }
    else if cs.contains_key("green"){
        if cs.get("green").unwrap() <=green_cubes{
            return 1
        }
        else{
            return -9
        }
    }
    else{
        0
    }
}

fn main() {

    let games_string = read_to_string("./src/input.txt").unwrap();
    let games = games_string.split("\n").collect::<Vec<_>>();

    let games_cubes = games.iter().map(|x| -> Vec<Vec<HashMap<String,u16>>>{
        x.split(':').collect::<Vec<_>>()[1].to_string().split(";").map(|x|->Vec<HashMap<String,u16>> {    
                    let vec_hashmap = x.split(",").collect::<Vec<_>>().iter().map(|x| -> HashMap<String,u16> {
                        let temp_vec = x.split(" ").collect::<Vec<_>>();
                        let mut hm:HashMap<String,u16> = HashMap::new();
                        hm.insert(temp_vec[2].to_string(), temp_vec[1].parse::<u16>().unwrap());
                        hm
                    }).collect::<Vec<_>>();
                    vec_hashmap
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let red_cubes:u16=12;
    let blue_cubes:u16=14;
    let green_cubes:u16=13;

    let evaluated_games = games_cubes.iter().enumerate().map(|(i, game)| -> Vec<u16> {
        game.iter().map(|cube_set| -> u16 {
            let res = cube_set.iter().map(|x| check_cubes(x, &red_cubes,&blue_cubes,&green_cubes)).collect::<Vec<i16>>().into_iter().reduce(|a,b| a+b).unwrap();
            if res>0{
                1
            }
            else{
                0
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    //evaluated_games.iter().for_each(|x| println!("{:?}", x));

    let result = evaluated_games.iter().map(|z| z.iter().any(|&x| x==0)).collect::<Vec<_>>();
    //result.iter().for_each(|x| println!("{:?}", x));
    let final_processing = result.iter().enumerate().filter(|(_i, &x)| x==false).map(|(i, x)| i+1).reduce(|a,b| a+b).unwrap();
    println!("{}", final_processing)
}
