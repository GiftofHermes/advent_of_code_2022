use std::fs;


fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}

fn main() {
    let data = read_file("data/test_input").expect("manually added file");
    
    let mut index = 0;
    let mut elf_calories = vec![0];

    for mut line in data.split("\n") { 
        line = line.trim();

        if line == "" { 
            index += 1;
            elf_calories.push(0);
        } else {
            let number = line.trim().parse::<i32>().unwrap();
            elf_calories[index] += number;
        }
    }

    elf_calories.sort_by(|a, b| b.cmp(a));

    dbg!(elf_calories[0], elf_calories[1], elf_calories[2]);
    dbg!(elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
