use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug,Clone)]
struct LightBeam{
    direction: String,
    row: usize,
    column: usize,
    col_limit: usize,
    row_limit: usize,
    has_finished: bool
}

impl LightBeam{
    fn move_it(&mut self, direction:&str){
        // Mark as finished if lightbeam reach a edge
        if (direction == "left" && self.column == 0) || 
        (direction == "right" && self.column==self.col_limit-1) ||
        (direction == "up" && self.row == 0) ||
        (direction == "down" && self.row == self.row_limit-1)
        {
            self.direction = direction.to_string();
            self.has_finished = true;
        }

        // Move the beam
        if self.has_finished == false 
        {
            if direction == "left" || direction == "right"
            {
                self.column = match direction {
                    "left" => self.column - 1,
                    "right" => self.column + 1,
                    _ => panic!("Unknown direction! Input direction was {}", direction)
                }
            }
            else
            {
                self.row = match direction {
                    "up" => self.row -1,
                    "down" => self.row + 1,
                    _ => panic!("Unknown direction! Input direction was {}", direction)
                }
            }

            self.direction = direction.to_string();
        }
    }

}


fn main() {
    let file_read = read_to_string("./src/input.txt").unwrap();

    let mirror_map = file_read.split("\n");
    let mut mirror_char_map:Vec<Vec<char>>=Vec::new();

    // Build char-map to evaluate by character
    for row in mirror_map{
        let mut charvec:Vec<char>=Vec::new();
        for c in row.chars(){
            charvec.push(c)   
        }
        mirror_char_map.push(charvec);
    }

    let col_limit = mirror_char_map[0].len();
    let row_limit= mirror_char_map.len();

    // build light_map we're going to update
    let mut light_map = vec![vec!['.'; col_limit];row_limit];
    println!("{:?}", mirror_char_map[0]);

    // We need to keep track of light beans, and also define what's one of those
    // let's go for a Vec<LightBeam>
    let mut lightbeam_list = Vec::<LightBeam>::new();

    let mut dashed_mirrors_hit: Vec<(usize,usize,String)> = Vec::new();
    let mut flat_mirrors_hit: Vec<(usize,usize,String)> = Vec::new();

    //positions_walked.insert((0,0,"right".to_string()));

    let first_lightbeam: LightBeam = LightBeam{
        direction: "right".to_string(),
        row: 0,
        column: 0,
        col_limit: col_limit,
        row_limit: row_limit,
        has_finished: false
    };

    lightbeam_list.push(first_lightbeam);

    let mut lighbeam_to_process = lightbeam_list.iter().any(|l| l.has_finished==false);
    
    while lighbeam_to_process{
        let mut extra_lightbeams = Vec::<LightBeam>::new();

        let lighbeams_loop = lightbeam_list.iter_mut().filter(|l| l.has_finished==false);

        for lb in lighbeams_loop{
            // Get current symbol on the map
            let symbol = mirror_char_map[lb.row][lb.column];
            // In any case it will be lit up. mark it on the light map
            light_map[lb.row][lb.column] = '#';

            // This part just moves the lightbean to the next position
            // as move_it already considers constraints we just need to keep moving on
            if symbol=='\\'{
                if lb.direction=="right"{
                    lb.move_it("down");
                }
                else if lb.direction=="left"{
                    lb.move_it("up");

                }
                else if lb.direction=="up"{
                    lb.move_it("left");

                }
                else{
                    lb.move_it("right");
                }

                if dashed_mirrors_hit.iter().any(|x| x.0==lb.row && x.1==lb.column && x.2==lb.direction){
                    lb.has_finished=true
                }
                else{
                    dashed_mirrors_hit.push((lb.row,lb.column,lb.direction.to_string()));
                }
            }
            else if symbol=='/'{
                if lb.direction=="right"{
                    lb.move_it("up");
                }
                else if lb.direction=="left"{
                    lb.move_it("down");
                }
                else if lb.direction=="up"{
                    lb.move_it("right");
                }
                else{
                    lb.move_it("left");
                }

                if dashed_mirrors_hit.iter().any(|x| x.0==lb.row && x.1==lb.column && x.2==lb.direction){
                    lb.has_finished=true
                }
                else{
                    dashed_mirrors_hit.push((lb.row,lb.column,lb.direction.to_string()));
                }
            }
            else if symbol=='|'{
                if lb.direction=="right" || lb.direction=="left"{     
                    // Create a extra lightbean
                    let mut extra_lightbeam = lb.clone();
                    // One lightbeam moves up
                    lb.move_it("up");
                    extra_lightbeam.move_it("down");

                    if flat_mirrors_hit.iter().any(|x| x.0==lb.row && x.1==lb.column && (x.2=="right" || x.2=="left")){
                        lb.has_finished=true
                    }
                    else{
                        flat_mirrors_hit.push((lb.row,lb.column,lb.direction.to_string()));
                    }

                    if flat_mirrors_hit.iter().any(|x| x.0==extra_lightbeam.row && x.1==extra_lightbeam.column && (x.2=="up" || x.2=="down")){
                        extra_lightbeam.has_finished=true
                    }
                    else{
                        flat_mirrors_hit.push((extra_lightbeam.row,extra_lightbeam.column,extra_lightbeam.direction.to_string()));
                    }

                    extra_lightbeams.push(extra_lightbeam);
                }
                else{
                    let same_direction:String = lb.direction.clone();
                    lb.move_it(same_direction.as_str());
                }
            }
            else if symbol=='-'{
                if lb.direction=="up" || lb.direction=="down"{
                    // Create a extra lightbean and move it down
                    let mut extra_lightbeam = lb.clone();
                    // One lightbeam moves up
                    lb.move_it("left");
                    extra_lightbeam.move_it("right");




                    extra_lightbeams.push(extra_lightbeam);
                }
                else{
                    let same_direction:String = lb.direction.clone();
                    lb.move_it(same_direction.as_str());
                }
                
            }
            else{
                // Else just keep moving on the same direction
                lb.move_it(lb.direction.clone().as_str());
            }
        }
        lightbeam_list.extend(extra_lightbeams);
        lightbeam_list.retain(|x| x.has_finished==false);

        // refresh lightbeam check
        lighbeam_to_process = lightbeam_list.iter().any(|l| l.has_finished==false);
    }
    // PROGRAM
    // Start the first LightBeam at 0,0
    // Check what exists on mirror_char_map -->
    //// If it's a DOT, mark light_map with # on the same position
    //// It it's a \\, mark light_map with # on the same position + check set of rules of this symbol to return direction
    //// If it's a 
    let mut total_enlighted: u32 = 0;

    println!("Lightbeams: {:?}", lightbeam_list);
    println!("Map:");
    for r in mirror_char_map.into_iter(){
        println!("{:?}", r);
    }
    println!("Lightmap:");
    for r in light_map.into_iter(){
        total_enlighted += r.iter().filter(|x| x==&&'#').count() as u32;
        println!("{:?}", r);
    }
    println!("Total enlighted: {}", total_enlighted)
}
