use std::collections::HashMap;

use regex::Regex;

// PageSizeInMm(width, height)
#[derive(Debug, Clone)]
pub struct PageSizeInMm(pub f64, pub f64);

impl PageSizeInMm {
    pub fn new(pagesize: &str) -> Self {
        let pagesize = pagesize.to_lowercase().trim().to_string();
        let pagesize = pagesize.as_str();
        let page_size_map = HashMap::from([
            ("a0", PageSizeInMm(841.0, 1189.0)),
            ("a1", PageSizeInMm(594.0, 841.0)),
            ("a2", PageSizeInMm(420.0, 594.0)),
            ("a3", PageSizeInMm(297.0, 420.0)),
            ("a4", PageSizeInMm(210.0, 297.0)),
            ("a5", PageSizeInMm(148.0, 210.0)),
            ("a6", PageSizeInMm(105.0, 148.0)),
            ("b0", PageSizeInMm(1000.0, 1414.0)),
            ("b1", PageSizeInMm(707.0, 1000.0)),
            ("b2", PageSizeInMm(500.0, 707.0)),
            ("b3", PageSizeInMm(353.0, 500.0)),
            ("b4", PageSizeInMm(250.0, 353.0)),
            ("b5", PageSizeInMm(176.0, 250.0)),
            ("b6", PageSizeInMm(125.0, 176.0)),
            ("jb0", PageSizeInMm(1030.0, 456.0)),
            ("jb1", PageSizeInMm(728.0, 1030.0)),
            ("jb2", PageSizeInMm(515.0, 728.0)),
            ("jb3", PageSizeInMm(364.0, 515.0)),
            ("jb4", PageSizeInMm(257.0, 364.0)),
            ("jb5", PageSizeInMm(182.0, 257.0)),
            ("jb6", PageSizeInMm(128.0, 182.0)),
            ("letter", PageSizeInMm(215.9, 279.4)),
            ("legal", PageSizeInMm(215.9, 355.6)),
            ("tabloid", PageSizeInMm(279.4, 431.8)),
        ]);

        if page_size_map.contains_key(pagesize) {
            return page_size_map.get(pagesize).unwrap().to_owned();
        }

        if pagesize.ends_with("^t") {
            let pdf_format = pagesize.split('^').nth(0).unwrap();
            if !page_size_map.contains_key(pdf_format) {
                panic!(
                    "PDF format {} is not recognized. Run -h/--help to see valid pagesize value.",
                    pdf_format
                );
            };
            return page_size_map.get(pdf_format).unwrap().invert();
        }

        let float_regex_str = r"\d+(\.\d+)?";
        let float_size_regex_str = format!("{}(?:mm|cm|in)", float_regex_str);
        let pagesize_regex_str = format!("^{}x{}$", float_size_regex_str, float_size_regex_str);

        let customized_pagesize_regex = Regex::new(pagesize_regex_str.as_str()).unwrap();

        if customized_pagesize_regex.is_match(pagesize) {
            let mut pagesize_split = pagesize.split('x');
            let width_str = pagesize_split.next().unwrap();
            let height_str = pagesize_split.next().unwrap();

            let get_size_in_mm = |size_str: &str| {
                let size_num = &size_str[..size_str.len() - 2].parse::<f64>().unwrap();
                let size_unit = &size_str[(size_str.len() - 2)..];

                match size_unit {
                    "mm" => *size_num,
                    "cm" => size_num * 10.0,
                    "in" => size_num * 25.4,
                    _ => panic!("Invalid unit"),
                }
            };
            return PageSizeInMm(get_size_in_mm(width_str), get_size_in_mm(height_str));
        }

        panic!(
            "Pagesize value {} is invalid. Run -h/--help to see valid pagesize value.",
            pagesize
        );
    }

    pub fn invert(&self) -> Self {
        let PageSizeInMm(width, height) = *self;
        PageSizeInMm(height, width)
    }
}
