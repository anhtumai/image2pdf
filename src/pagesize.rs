#[derive(Debug)]
pub struct PageSize {
    pub width: f64,
    pub height: f64,
}

impl PageSize {
    pub fn new(page_size: &str) -> Self {
        match page_size {
            "A4" => PageSize {
                width: 210.0,
                height: 297.0,
            },
            _ => panic!("Not implemented"),
        }
    }
}
