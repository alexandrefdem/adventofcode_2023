use std::{fs::read_to_string};

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

fn get_lens_power(boxes:Vec<Box>, mut total_focusing_power:u32){
    for box_unit in boxes.iter(){
        for (i, lens_on_box) in box_unit.lenses.iter().enumerate(){
            total_focusing_power =  total_focusing_power + (1+box_unit.id)*(i as u32+1)*lens_on_box.focal;
        }
    }
    println!("Total focusing power {}", total_focusing_power);
}


#[derive(Debug)]
struct Lens{
    label:String,
    operation: char,
    focal: u32
}

#[derive(Debug)]
struct Box {
    id:u32,
    lenses: Vec<Lens>
}

fn main() {
    let mut boxes:Vec<Box> = Vec::new();

    for s in read_to_string("./src/test.txt").unwrap().split(',').into_iter(){
        let instruction = s.to_string();

        // Instruction is add
        if instruction.contains("="){
            //println!("Instruction to Add lens!");
            let label_focal:Vec<_> = instruction.split("=").collect();
            let given_box: u32 = hash_algorithm(label_focal[0]);
            //let focal = check_numeric(label_focal[1]);
            let focal: u32 = match label_focal[1].to_string().parse::<u32>(){
                Ok(v) => v,
                Err(e) => 0
            };

            println!("{:?} {} {}", label_focal, given_box, focal);

            let lens: Lens = Lens{
                label: label_focal[0].to_string(),
                operation: '=',
                focal: focal
            };
            
            if boxes.iter().any(|b| b.id == given_box){
                // If box exists
                let box_index = boxes.iter().position(|b| b.id==given_box).unwrap();
                if boxes[box_index].lenses.iter().any(|b| b.label == lens.label){
                    let lens_index = boxes[box_index].lenses.iter().position(|b| b.label == lens.label).unwrap();
                    boxes[box_index].lenses[lens_index] = lens;
                }
                else {
                    boxes[box_index].lenses.push(lens);
                }
            }
            else{
                //Given box does not exist
                let mut vec_lenses:Vec<Lens> = Vec::new();
                vec_lenses.push(lens);
                let newbox:Box = Box { id: given_box, lenses: vec_lenses};
                boxes.push(newbox);
            }
            
        }
        else {
            //Instruction is Remove
            //println!("Instruction to Remove lens!");
            let label_focal:Vec<_> = instruction.split("-").collect();
            let label = label_focal[0];
            let given_box = hash_algorithm(label);

            if boxes.iter().any(|b| b.id == given_box){
                let box_index = boxes.iter().position(|b| b.id==given_box).unwrap();
                if boxes[box_index].lenses.iter().any(|l| l.label == label){
                    let lens_index = boxes[box_index].lenses.iter().position(|b| b.label == label).unwrap();
                    boxes[box_index].lenses.remove(lens_index);
                }
            }
            
        }
    }

    /*for b in &boxes{
        println!("{:?}", b);
    }*/

    let mut total_focusing_power: u32 = 0;
    get_lens_power(boxes, total_focusing_power);

}
