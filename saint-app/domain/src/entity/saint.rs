use uuid::Uuid;

pub(crate) struct Saint {
    pub(crate) id: Option<Uuid>,
    pub(crate) display_name: Option<String>,
    pub(crate) english_name: Option<String>,
    pub(crate) french_name: Option<String>,
    pub(crate) latin_name: Option<String>,
    pub(crate) vietnamese_name: Option<String>,
    pub(crate) gender: Option<String>,
    pub(crate) feast_day: Option<String>,
}

impl Saint {
    pub(crate) fn is_valid(&self) -> bool {
        println!("checking if saint is valid");
        self.display_name.is_some()
            && self.vietnamese_name.is_some()
            && self.feast_day.is_some()
            && self.gender.is_some()
    }
}
