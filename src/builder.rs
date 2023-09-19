use crate::parser::{ExecutionType, ParsedArg};

trait PushString {
    fn push_string(&mut self, string: String);
}

impl PushString for String {
    fn push_string(&mut self, string: String) {
        self.push_str(string.as_str());
    }
}

#[derive(Debug)]
pub enum BuildingErr {
    UnreconizedArg,
}

#[derive(Debug)]
pub struct Cmd {
    pub invocation: String,
    pub args: Vec<String>,
}

struct CrfRateBuf {
    crf: u32,
    max_rate: f32,
    buf_size: f32,
}

/*

QUALITY_MAX = 100
CRF_MAX = 51
BITRATE_LOW = 0.5  # BITRATE_IN_MB_AT_0_QUALITY
BITRATE_HIGH = 12  # BITRATE_IN_MB_AT_100_QUALITY
BITRATE_QUALITY_CURVE_FACTOR = 2.2

def quality_to_crf(quality: int) -> int:
    crf = int((QUALITY_MAX - quality) / (QUALITY_MAX / CRF_MAX))
    return cramp_between(crf, 0, CRF_MAX)
    */
const QUALITY_MAX: u32 = 100;
const CRF_MAX: u32 = 51;
const BITRATE_LOW: f32 = 0.5;
const BITRATE_HIGH: u32 = 12;
const BITRATE_QUALITY_CURVE_FACTOR: f32 = 2.2;

fn quality_to_crf(quality: u32) -> CrfRateBuf {
    let bitrate = f32::round(
        BITRATE_LOW
            + (quality as f32).powf(BITRATE_QUALITY_CURVE_FACTOR)
                / (BITRATE_QUALITY_CURVE_FACTOR * 1000.0),
    );

    CrfRateBuf {
        crf: (QUALITY_MAX - quality) / (QUALITY_MAX / CRF_MAX),
        max_rate: bitrate,
        buf_size: 2.0 * bitrate,
    }
}

pub fn build_args(args: Vec<ParsedArg>) -> Result<Cmd, BuildingErr> {
    let mut cmd = Cmd {
        invocation: "ffmpeg".to_string(),
        args: Vec::new(),
    };

    let mut filters_builded = Vec::<String>::new();
    let mut input_files_builded = Vec::<String>::new();
    let mut quality_builded = String::new();
    let mut map_builded = String::new();

    for arg in args {
        match arg {
            ParsedArg::Resize(dimensions) => {
                filters_builded.push(format!("scale={}:{}", dimensions.0, dimensions.1));
            }
            ParsedArg::Quality(quality) => {
                let crf = quality_to_crf(quality);
                quality_builded.push_string(format!(
                    "-crf {} -maxrate {}M -bufsize {}M",
                    crf.crf, crf.max_rate, crf.buf_size
                ));
            }
            ParsedArg::Merge(filepaths) => {
                let (filter, map) = build_merge(&filepaths);
                filters_builded.push(filter);
                map_builded = map;
            }
            ParsedArg::OutFilePath(path) => input_files_builded.push(path),
            _ => panic!("Argument {:?} is not reconized", arg),
        };
    }

    if filters_builded.len() > 0 {
        cmd.args.push(build_merged_filters(filters_builded));
    }
    if quality_builded.len() > 0 {
        cmd.args.push(quality_builded);
    }
    if map_builded.len() > 0 {
        cmd.args.push(map_builded)
    }
    Ok(cmd)
}

fn build_merged_filters(filters: Vec<String>) -> String {
    format!("-vf {}", filters.join(" ,"))
}

fn build_merge(filepaths: &Vec<String>) -> (String, String) {
    let mut builded_filepaths = Vec::<String>::new();
    let mut builded_concat_input = Vec::<String>::new();
    for i in 0..filepaths.len() {
        builded_filepaths.push(format!("[{}][v{}]", i, i));
        builded_concat_input.push(format!("[v{}][{}:a:0]", i, i))
    }

    let builded_filter = format!(
        "{};{}concat=n={}:v=1:a=1[outv][outa]",
        builded_filepaths.join(";"),
        builded_concat_input.join(""),
        filepaths.len()
    );
    let builded_map = format!("-map \"n[outv]\" -map \"[outa]\" -vsync 0");

    (builded_filter, builded_map)
}

/*
fn build_args_metadata(metadata: Vec<ParsedMetadata>) -> Vec<String> {
    let mut in_filepath = String::new();
    let mut out_filepath = String::new();
    for metadata in args.metadata {
        match metadata {
            ParsedMetadata::ExecutionType(execution_type) => match execution_type {
                ExecutionType::Default => cmd.invocation = "ffmpeg".to_string(),
                ExecutionType::Preview => cmd.invocation = "ffplay".to_string(),
            },
            ParsedMetadata::InFilePath(fp) => in_filepath = fp,
            ParsedMetadata::OutFilePath(fp) => out_filepath = fp,
        }
    }
}

fn build_args_options(metadata: Vec<ParsedMetadata>) -> Vec<String> {
    let mut options = Vec::<String>::new();
    for option in args.options {
        match option {
            ParsedOption::Quality(value) => {
                let crf_rate_buf = quality_to_crf(value);
                options.push(format!(
                    "-crf {} -maxrate {}M -bufsize {}M",
                    crf_rate_buf.crf, crf_rate_buf.max_rate, crf_rate_buf.buf_size
                ));
            }
        }
    }
}

fn build_args_filters(filters: Vec<ParsedFilter>) -> Vec<String> {
    let mut builded_filters = Vec::<String>::new();
    for filter in filters {
        match filter {
            ParsedFilter::Resize(dimensions) => {
                builded_filters.push(format!("scale={}:{}", dimensions.0, dimensions.1))
            },
            ParsedFilter::BommerangueGif((start, duration)) => {
                builded_filters.push(format!("[0]crop=360:640,split[c1][c2];[c1]trim=start_frame=1:end_frame={duration}*29.98,setpts=PTS-STARTPTS,reverse[r];[c2][r]concat=n=2:v=1:a=0,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse"))
            }
        }
    }
    if builded_filters.len() > 0 {
        vec![format!(
            "-filter_complex \"{}\"",
            builded_filters.join(", ")
        )]
    } else {
        vec!["".to_owned()]
    }
}
*/
