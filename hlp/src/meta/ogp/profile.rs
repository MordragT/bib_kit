#[derive(Debug)]
pub struct OgpProfile {
    /// A name normally given to an individual by a parent or self-chosen.
    pub first_name: Option<String>,
    ///  A name inherited from a family or marriage and by which the individual is commonly known.
    pub last_name: Option<String>,
    /// A short unique string to identify them.
    pub username: Option<String>,
    /// Their gender.
    pub gender: Option<String>,
}
