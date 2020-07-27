use crate::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AreAllDisjoint<T = ()> {
    No(T),
    Yes,
}

impl AreAllDisjoint {
    pub const YES: Self = AreAllDisjoint::Yes;
    pub const NO: Self = AreAllDisjoint::No(());
}

impl AreAllDisjoint<()> {
    pub const fn with_dummy_error(self) -> AreAllDisjoint<Error> {
        match self {
            AreAllDisjoint::No(()) => {
                let err = Error::OverlappingIndexArgs {
                    left: u16::MAX,
                    right: u16::MAX,
                };
                AreAllDisjoint::No(err)
            }
            AreAllDisjoint::Yes => AreAllDisjoint::Yes,
        }
    }
}
impl AreAllDisjoint<Error> {
    pub const fn check_is_expected<T>(self, expected: &AreAllDisjoint<T>) -> Result<(), Error> {
        match (self, expected) {
            (AreAllDisjoint::Yes, _) => Ok(()),
            (_, AreAllDisjoint::No(_)) => Ok(()),
            (AreAllDisjoint::No(x), AreAllDisjoint::Yes) => Err(x),
        }
    }
}
