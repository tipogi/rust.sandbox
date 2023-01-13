const PLUS:char = '+';

pub fn search_next_plus(splitted_array: &str) -> isize {
    println!("Searching next +, {}", splitted_array);
    let index = splitted_array.find('+');
    match index {
        Some(x) => x as isize + 1,
        None => -1
    }
}

pub fn find_square(top_left_corner: usize, top_right_corner: usize, other_lines: &[&str], squares: &mut u32) {
    println!("Searching square {} - {}", top_left_corner, top_right_corner );
    let mut index = 0;
    let mut vertical_bars:Vec<char> = Vec::new();
    for line in other_lines {
        let vector_char:Vec<char> = line.chars().collect();
        if vector_char[top_left_corner] == PLUS && vector_char[top_right_corner] == PLUS {
            if has_horizontal_bar(top_left_corner, top_right_corner, line) {
                if search_vertical_bars(&vertical_bars) {
                    println!("===> MATCH: {}, {} in line {}", top_left_corner, top_right_corner, index + 1);
                    *squares += 1;
                    println!("done, square search");
                }
            }
        } else {
            vertical_bars.push(vector_char[top_left_corner]);
            vertical_bars.push(vector_char[top_right_corner]);
        }
        index += 1;
    }
}

pub fn has_horizontal_bar(top_left_corner: usize, top_right_corner: usize, top_line_square: &str) -> bool {
    let vector_char:Vec<char> = top_line_square.chars().collect();
    let character_diff_between = top_right_corner - top_left_corner;
    println!("Has {}, L:{}, R:{}", character_diff_between, top_left_corner, top_right_corner);
    if character_diff_between == 2 {
        return vector_char[top_right_corner -1] == '-';
    } else {
        let mut start_checking = false;
        let closed = true;
        for (index, line) in top_line_square.chars().enumerate() {
            if index == top_left_corner {
                println!("closing line: START checking");
                start_checking = true;
            } else if index == top_right_corner {
                println!("closing line: FINISH checking");
                start_checking = false;
            } else if start_checking && line != '-' && line != '+'{
                println!("closing line: NOT CLOSE, char = {}, index = {}", line, index);
                return false;
            } 
        }
        println!("{}", closed);
        return closed;
    }
}

fn search_vertical_bars(vertical_bars: &Vec<char>) -> bool {
    println!("----------------- {:?}", vertical_bars);
    for bar in vertical_bars.iter() {
        if *bar != '|' && *bar != '+' {
            return false
        }
    }
    true
}

pub fn count_first_iteration(lines: &[&str]) -> u32 {
    let mut line = 0;
    let mut squares = 0;
    // Loop line by line
    while line < lines.len() {
        let char_line = lines[line];
        println!("line:{:?}", char_line);
        // Loop each line plus characters
        for (top_left_corner, ascii_char) in char_line.chars().enumerate() {
            println!("COORDS: ({})", top_left_corner);
            if ascii_char == '+' {
                let after_plus_characters = char_line.split_at(top_left_corner + 1).1;
                // Loop after plus search, next plus character
                for (next_plus_index, next_plus) in after_plus_characters.chars().enumerate() {
                    if next_plus == '+' {
                        let top_right_corner = next_plus_index + top_left_corner + 1;
                        // Check if the line has horizontal bars (-) to close the line between corners
                        if has_horizontal_bar(top_left_corner, top_right_corner, char_line) {
                            println!("closed horizontal bar");
                            find_square(top_left_corner, top_right_corner, lines.split_at(line + 1).1, &mut squares);
                        }
                        
                    }
                }
            }
        }
        line += 1;
        println!("new line: {}", line);
    }
    squares
    
}

fn main() {
    let ascii_diagram_a= [
        "+---+--+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    let ascii_diagram_b = [
        "  +-+", 
        "    |", 
        "+-+-+", 
        "| | -", 
        "+-+-+"
    ];
    let ascii_diagram_c = [
        "  + +  ",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+"
    ];
    let ascii_diagram_d = [
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+"
    ];
    let ascii_diagram_e = [
        "+------+----+", 
        "|      |    |", 
        "+---+--+    |", 
        "|   |       |", 
        "+---+-------+"
    ];
    let squares = count(&ascii_diagram_e);
    println!("Squares has to be 3 and it is: {}", squares);
}