pub struct Group<'a> {
    sack1: &'a str,
    sack2: &'a str,
    sack3: &'a str,

}

impl<'a> Group<'_> { 
    pub fn new(sack1: &'a str, sack2: &'a str, sack3: &'a str) -> Group<'a> {
        Group {
            sack1,
            sack2,
            sack3 
        } 
    }

    pub fn find_badge(self) -> Option<char> { 
        for c in self.sack1.chars() { 
            if self.sack2.contains(c) && self.sack3.contains(c) { 
                return Some(c);
            }
        }
        None
    }
}
