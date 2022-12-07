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
    section1: Section,
    section2: Section,
}

impl CleaningPlan {
    fn new(section1: Section, section2: Section) -> CleaningPlan {
        CleaningPlan { section1, section2 }
    }

    pub fn is_fully_encompassing(self) -> bool {
        if self.section1.lo >= self.section2.lo && self.section1.hi <= self.section2.hi {
            return true;
        }
        if self.section1.lo <= self.section2.lo && self.section1.hi >= self.section2.hi {
            return true;
        }
        false
    }

    pub fn is_overlapping(self) -> bool {
        let maxmin = if self.section1.lo > self.section2.lo {
            self.section1.lo
        } else {
            self.section2.lo
        };
        let minmax = if self.section1.hi < self.section2.hi {
            self.section1.hi
        } else {
            self.section2.hi
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
