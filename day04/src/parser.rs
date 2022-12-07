struct Section {
    lo: u32,
    hi: u32,
}

impl Section {
    fn new(lo: u32, hi: u32) -> Section {
        Section { lo, hi }
    }
}

pub struct CleaningPlan {
    Section1: Section,
    Section2: Section,
}

impl CleaningPlan {
    fn new(Section1: Section, Section2: Section) -> CleaningPlan {
        CleaningPlan { Section1, Section2 }
    }

    pub fn is_fully_encompassing(self) -> bool {
        if self.Section1.lo >= self.Section2.lo && self.Section1.hi <= self.Section2.hi {
            return true;
        }
        if self.Section1.lo <= self.Section2.lo && self.Section1.hi >= self.Section2.hi {
            return true;
        }
        false
    }

    pub fn is_overlapping(self) -> bool {
        let maxmin = if self.Section1.lo > self.Section2.lo {
            self.Section1.lo
        } else {
            self.Section2.lo
        };
        let minmax = if self.Section1.hi < self.Section2.hi {
            self.Section1.hi
        } else {
            self.Section2.hi
        };

        if maxmin <= minmax {
            true
        } else {
            false
        }
    }
}

fn parse_section(section: &str) -> Section {
    let indexes: Vec<&str> = section.split('-').collect();
    let lo = indexes[0].parse::<u32>().unwrap(); // in the puzzle must be uint
    let hi = indexes[1].parse::<u32>().unwrap(); // in the puzzle must be uint

    Section::new(lo, hi)
}

pub fn parse(line: &str) -> CleaningPlan {
    let sections: Vec<&str> = line.split(',').collect();
    let section1 = sections[0];
    let section2 = sections[1];

    CleaningPlan::new(parse_section(section1), parse_section(section2))
}
