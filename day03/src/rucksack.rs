pub struct Rucksack<'a> { 
    compartment1: &'a str,
    compartment2: &'a str   
}

impl Rucksack<'_> { 
    pub fn new(data: &str) -> Rucksack { 
        //let data = data.to_string();
        let str_length = data.chars().count();
        let mid_point = str_length/2;
        Rucksack{
            compartment1: &data[0..mid_point],
            compartment2: &data[mid_point..str_length]
        }
    }

    pub fn shared_items(&self) -> Vec<char> { 
        let mut shared: Vec<char> = vec![];
        for c in self.compartment1.chars() { 
            if self.compartment2.contains(c) && !shared.contains(&c) { 
                shared.push(c)
            }
        }
        
        shared
    }
}

pub fn char_to_point(item: char) -> u32 {
    let first_none_letter_ascii_character = 91;
    let length_of_special_ascii_characters = 6;
    let index_of_char_a = 97;
    let mapping_space_length = 52;

    let mut point = item as u32;
    if point < first_none_letter_ascii_character {
        point += length_of_special_ascii_characters;
    }
    point = (point + mapping_space_length - index_of_char_a + 1) % mapping_space_length;
    if point == 0 { point += mapping_space_length}
    point
}