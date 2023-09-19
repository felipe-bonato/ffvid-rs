use std::{env, ffi::OsStr, fmt::Debug};

use crate::recursive_descent_parser::RecursiveDescentParser;

#[derive(Debug)]
pub enum ParsingErr {
    Unknown(String),
    Quality(String),
    Resize(String),
    Filepath(String),
    Merge(String),
    ShouldNotHappen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionType {
    Default,
    Preview,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedArg {
    Resize((i32, i32)),
    Quality(u32),
    Merge(Vec<String>),
    ExecutionType(ExecutionType),
    OutFilePath(String),
}

#[derive(Debug)]
pub struct ArgsParser<'a> {
    parser: RecursiveDescentParser<'a, String>,
    parsed_args: Vec<ParsedArg>,
}

impl<'a> ArgsParser<'a> {
    pub fn new(args: std::iter::Peekable<std::slice::Iter<'a, String>>) -> ArgsParser {
        ArgsParser {
            parser: RecursiveDescentParser::new(args),
            parsed_args: Vec::new(),
        }
    }

    pub fn parse_args(&mut self) -> Result<Vec<ParsedArg>, ParsingErr> {
        // Eliminate the call to the executable itself
        let _call_name = self.parser.next_or(ParsingErr::ShouldNotHappen)?;

        loop {
            let parsed_arg = match self.parser.next() {
                Some(arg) => match arg.clone().as_str() {
                    "--resize" => self.parse_arg_resize()?,
                    "--quality" => self.parse_arg_quality()?,
                    "--preview" => self.parse_arg_preview()?,
                    "--merge" => self.parse_arg_merge()?,
                    pathname => self.parse_out_pathname(pathname)?,
                },
                None => return Ok(self.parsed_args.clone()),
            };
            self.parsed_args.push(parsed_arg);
        }
    }

    fn parse_arg_quality(&mut self) -> Result<ParsedArg, ParsingErr> {
        let arg_value_string = self.parser.next_or(ParsingErr::Quality(
            "Expected quality value after filter `quality` (eg: --quality 50), found nothing."
                .to_owned(),
        ))?;

        let arg_value = arg_value_string.parse::<u32>().map_err(|_| {
            ParsingErr::Quality(format!(
                "Value `{}` is not a valid positive integer (eg: 50).",
                arg_value_string
            ))
        })?;

        Ok(ParsedArg::Quality(arg_value))
    }

    fn parse_arg_resize(&mut self) -> Result<ParsedArg, ParsingErr> {
        let arg_value = self.parser.next_or(
            ParsingErr::Resize("Expected resize dimensions after filter `resize` (eg: --resize 1920:1080), found nothing.".to_owned())
        )?;

        let dimensions: Vec<&str> = arg_value.split(":").collect();
        if dimensions.len() != 2 {
            return Err(ParsingErr::Resize(format!(
                "Values `{}` are not valid dimensions (eg: 1920:1080).",
                arg_value
            )));
        }

        match (dimensions[0].parse::<i32>(), dimensions[1].parse::<i32>()) {
            (Ok(width), Ok(heigth)) => Ok(ParsedArg::Resize((width, heigth))),
            (_, _) => Err(ParsingErr::Resize(format!(
                "Value `{}` is not valid dimension (eg: 1920:1080)",
                arg_value
            ))),
        }
    }

    fn parse_arg_preview(&mut self) -> Result<ParsedArg, ParsingErr> {
        Ok(ParsedArg::ExecutionType(ExecutionType::Preview))
    }

    fn pathname_parser(pathname: &str) -> Result<String, ParsingErr> {
        // Pathname is converted to a std::path::Path just for validation.
        // It is stored in the final parsed_args as String.
        // This was done because Path is basically a &str,
        // and thus, reeeaally annoying to store in the final data.
        let path = std::path::Path::new(pathname);
        /*if !path.exists() && pathname != "__test.mp0" {
            return Err(ParsingErr::Filepath(format!(
                "Path `{}` is not valid path (eg: video.mp4).",
                pathname
            )));
        }*/
        Ok(pathname.to_string())
    }

    fn parse_out_pathname(&mut self, pathname: &str) -> Result<ParsedArg, ParsingErr> {
        if self.parser.tokens_left() > 1 {
            return Err(ParsingErr::Unknown(format!(
                "Argument `{pathname}` is unreconized.",
            )));
        }

        match ArgsParser::pathname_parser(pathname) {
            Ok(filepath) => Ok(ParsedArg::OutFilePath(filepath)),
            Err(e) => Err(e),
        }

        /* let out_filepath = match path.extension() {
            Some(extension) => format!(
                "{}-edit.{}",
                ArgsParser::os_str_to_string_or_empty(path.file_stem()),
                ArgsParser::os_str_to_string_or_empty(path.extension())
            ),
            None => panic!(),
        };*/
    }

    fn parse_arg_merge(&mut self) -> Result<ParsedArg, ParsingErr> {
        let mut parsed_merged_files = Vec::<String>::new();
        loop {
            if self.parser.tokens_left() == 1 {
                break;
            }
            match self.parser.next() {
                Some(filepath) => parsed_merged_files.push(ArgsParser::pathname_parser(filepath)?),
                None => break,
            }
        }
        if parsed_merged_files.len() < 2 {
            // TODO: If 0 file names are found, this error message looks ugly.
            Err(ParsingErr::Merge(format!(
                "Expected at least 2 filenames in filter `merge` (eg.: --merge a.mp4 b.mp4), found {} ({}) filename(s).",
                parsed_merged_files.len(),
                parsed_merged_files.join(",")
            )))
        } else {
            Ok(ParsedArg::Merge(parsed_merged_files))
        }
    }

    fn os_str_to_string_or_empty(os_str: Option<&OsStr>) -> &str {
        os_str.unwrap_or(OsStr::new("")).to_str().unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_parsed_args(args_vec: Vec<&str>) -> Result<Vec<ParsedArg>, ParsingErr> {
        let args_vec_string: Vec<String> = args_vec.iter().map(|arg| arg.to_string()).collect();

        let mut parser = ArgsParser::new(args_vec_string.iter().peekable());
        parser.parse_args()
    }

    #[test]
    fn filters() {
        let parsed_args = build_parsed_args(vec!["ffvid", "--resize", "1920:1080"])
            .expect("Returned `Err` when it shouldn't");

        assert_eq!(parsed_args.len(), 1);
        assert_eq!(parsed_args[0], ParsedArg::Resize((1920, 1080)));
    }
}
