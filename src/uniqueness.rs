#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AreAllUnique {
    No = 0,
    Yes = 1,
}

impl AreAllUnique {
    pub const fn one_if_unique(self) -> usize {
        self as usize
    }

    pub const fn assert_equals(self, other: Self){
        ["At least one of the indices/range arguments overlaps with another one"]
        [(self as u8 != other as u8) as usize];
        
    }
}
