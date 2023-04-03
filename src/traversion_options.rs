

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TraversionOptions {
    Thili,
    Tohi,
    Sofi
}



impl Default for TraversionOptions {
    fn default() -> Self {
        Self::Tohi
    }
}
