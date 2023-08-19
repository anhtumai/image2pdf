use std::str::FromStr;

use anyhow::anyhow;
use colored::*;
use printpdf::Mm;
use regex::Regex;

macro_rules! page_size_formats {
    ($($name:ident => ($str:literal, $width:literal, $height:literal)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PageSizeFormat {
            $($name),*
        }

        impl FromStr for PageSizeFormat {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().trim().trim_end_matches("^t") {
                    $($str => Ok(PageSizeFormat::$name),)*
                    _ => Err(()),
                }
            }
        }

        impl PageSizeFormat {
            pub fn to_page_size(self) -> PageSizeInMm {
                match self {
                    $(PageSizeFormat::$name => PageSizeInMm {
                        width: Mm($width),
                        height: Mm($height),
                    }),*
                }
            }
        }

        impl From<PageSizeFormat> for PageSizeInMm {
            #[inline(always)]
            fn from(page_size_format: PageSizeFormat) -> Self {
                page_size_format.to_page_size()
            }
        }
    };
}

page_size_formats! {
    A0 => ("a0", 841.0, 1189.0),
    A1 => ("a1", 594.0, 841.0),
    A2 => ("a2", 420.0, 594.0),
    A3 => ("a3", 297.0, 420.0),
    A4 => ("a4", 210.0, 297.0),
    A5 => ("a5", 148.0, 210.0),
    A6 => ("a6", 105.0, 148.0),
    B0 => ("b0", 1000.0, 1414.0),
    B1 => ("b1", 707.0, 1000.0),
    B2 => ("b2", 500.0, 707.0),
    B3 => ("b3", 353.0, 500.0),
    B4 => ("b4", 250.0, 353.0),
    B5 => ("b5", 176.0, 250.0),
    B6 => ("b6", 125.0, 176.0),
    Jb0 => ("jb0", 1030.0, 456.0),
    Jb1 => ("jb1", 728.0, 1030.0),
    Jb2 => ("jb2", 515.0, 728.0),
    Jb3 => ("jb3", 364.0, 515.0),
    Jb4 => ("jb4", 257.0, 364.0),
    Jb5 => ("jb5", 182.0, 257.0),
    Jb6 => ("jb6", 128.0, 182.0),
    Letter => ("letter", 215.9, 279.4),
    Legal => ("legal", 215.9, 355.6),
    Tabloid => ("tabloid", 279.4, 431.8),
}

#[derive(Debug, Clone, Copy)]
pub struct PageSizeInMm {
    pub width: Mm,
    pub height: Mm,
}

impl PageSizeInMm {
    pub fn invert(self) -> Self {
        let PageSizeInMm { width, height } = self;
        PageSizeInMm {
            width: height,
            height: width,
        }
    }
}

impl FromStr for PageSizeInMm {
    type Err = anyhow::Error;

    fn from_str(pagesize: &str) -> Result<Self, Self::Err> {
        let pagesize = pagesize.to_lowercase();
        let pagesize = pagesize.trim();
        let pagesize_format = pagesize.parse::<PageSizeFormat>().ok();

        if let Some(pagesize_format) = pagesize_format {
            if pagesize.ends_with("^t") {
                return Ok(pagesize_format.to_page_size().invert());
            }
            return Ok(pagesize_format.into());
        }

        let float_regex_str = r"\d+(\.\d+)?";
        let float_size_regex_str = format!("{}(?:mm|cm|in)", float_regex_str);
        let pagesize_regex_str = format!("^{}x{}$", float_size_regex_str, float_size_regex_str);

        let customized_pagesize_regex = Regex::new(&pagesize_regex_str).unwrap();

        if customized_pagesize_regex.is_match(pagesize) {
            let mut pagesize_split = pagesize.split('x');
            let width_str = pagesize_split.next().unwrap();
            let height_str = pagesize_split.next().unwrap();

            fn get_size_in_mm(size_str: &str) -> anyhow::Result<f64> {
                let size_num = size_str[..size_str.len() - 2].parse::<f64>().unwrap();
                let size_unit = &size_str[(size_str.len() - 2)..];

                match size_unit {
                    "mm" => Ok(size_num),
                    "cm" => Ok(size_num * 10.0),
                    "in" => Ok(size_num * 25.4),
                    _ => Err(anyhow!(
                        "Invalid unit `{size_unit}`. Possible units: `mm`, `cm` and `in`."
                    )),
                }
            }
            return Ok(PageSizeInMm {
                width: Mm(get_size_in_mm(width_str)?),
                height: Mm(get_size_in_mm(height_str)?),
            });
        }

        anyhow::bail!(
            "Pagesize value {} is invalid. Run {} to see valid pagesize value.",
            pagesize.blue().underline(),
            "-h/--help".cyan()
        );
    }
}
