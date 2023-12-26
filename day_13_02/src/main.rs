use std::fs::read_to_string;

fn transpose_matrix<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> 
where T: Clone
{
    assert!(!v.is_empty());
    let mut new_matrix:Vec<Vec<T>>=Vec::new();

    let row_total = v.len();
    let col_total = v[0].len();

    for i in 0..col_total{
        let mut temp_vec :Vec<T> = Vec::new();
        for k in 0..row_total{
            temp_vec.push(v[k][i].clone());
        }
        new_matrix.push(temp_vec);
    }
    new_matrix
}

fn check_reflection(pattern: &Vec<String>) -> u32{
    let max_size = pattern.len() as i32 - 1;
    let mut cols_left_or_up:u32 = 0;

    for i in 0..max_size{

        let mut left_index:i32 = i;
        let mut right_index:i32 = i+1;

        while left_index>-1 && right_index<max_size+1{
            if pattern[left_index as usize] == pattern[right_index as usize]{
                left_index -= 1;
                right_index += 1;
            }
            else {
                break;
            }
        }
        if left_index==-1 || right_index==max_size+1{
            cols_left_or_up = i as u32 +1;
        }
    }
    return cols_left_or_up
}

fn fix_smudge(in_pattern: &Vec<String>)->Vec<String>
{
    let mut pattern = in_pattern.clone();
    let max_size = pattern.len() as i32 - 1;
    let mut found_mismatch = false;

    for i in 0..max_size{

        let mut left_index:i32 = i;
        let mut right_index:i32 = i+1;

        while left_index>-1 && right_index<max_size+1{
            if pattern[left_index as usize] == pattern[right_index as usize]{
                left_index -= 1;
                right_index += 1;
            }
            else {
                let mismatchs = pattern[left_index as usize].chars().zip(pattern[right_index as usize].chars()).filter(|(c1, c2)| c1 != c2).take(2).collect::<Vec<_>>();
                
                if mismatchs.len() == 1{
                    pattern[left_index as usize] = pattern[right_index as usize].clone();
                    found_mismatch = true;
                    break;
                }
                else{
                    left_index -= 1;
                    right_index += 1;
                }
            }
        }
        if found_mismatch{
            break;
        }
    }
    pattern
}

fn main() {
    // Read file as string, splits first by two newlines (means that we're between patterns)
    // Then splits again each pattern by single newline so we can get individual rows
    // Then split each row by it's chars.
    // The strategy I think of was to evaluate vertical rows as horizontal rows 
    // Because it's easier to test equality between Vec positions.
    // Therefore, I had to transpose the original matrix to turn each "column" into a row.
    let pattern_vec:Vec<Vec<Vec<char>>> = read_to_string("./src/test1.txt").unwrap().split("\n\n").into_iter()
        .map(|x: &str| x.split("\n").into_iter()
        .map(|charvec: &str| charvec.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>())
        .collect();

    // Transpose matrix to evaluate vertical rows as horizontal rows
    let transposed_pattern_vec:Vec<_> = pattern_vec.iter()
                                                    .map(|x| 
                                                        transpose_matrix(x)
                                                    ).collect();

    //Transform it back to string
    let mut original_patterns:Vec<Vec<String>> = pattern_vec.iter()
                                                .map(|x| 
                                                {
                                                    x.iter().map(|y| y.iter().collect()).collect()
                                                }).collect();

    // Fix smudge
    original_patterns = original_patterns.iter().map(|x| fix_smudge(x)).collect();

    let transposed_patterns: Vec<Vec<String>> = transposed_pattern_vec.iter()
                                                .map(|x| 
                                                    {
                                                        x.iter().map(|y| y.iter().collect()).collect()
                                                    }).collect();

    // Check reflections with check_reflection function, counting left/top mirrors
    let horizontal_reflections:u32= original_patterns.iter()
                                                    .map(|x| 
                                                        check_reflection(x)
                                                    ).collect::<Vec<u32>>().iter().sum();

    let vertical_reflections:u32= transposed_patterns.iter()
                                                    .map(|x| 
                                                        check_reflection(x)
                                                    ).collect::<Vec<u32>>().into_iter().sum();

    println!("{:?}", horizontal_reflections*100+vertical_reflections)
}
