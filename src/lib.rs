use anyhow::Context;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::cmp::Ordering;

pub fn update_file(sss: &String, file_path: &str) -> anyhow::Result<()> {
    let contents = {
        let mut contents = String::new();
        if let Ok(mut file) = std::fs::File::open(file_path) {
            file.read_to_string(&mut contents)
                .with_context(|| format!("could not read file `{}`", file_path))?;
        }
        contents
    };
    if contents != *sss {
        println!("update: {}", file_path);
        let mut file = std::fs::File::create(file_path)
            .with_context(|| format!("could not create file `{}`", file_path))?;
        write!(file, "{}", sss).with_context(|| format!("could not write file `{}`", file_path))?;
    }
    //
    Ok(())
}

#[rustfmt::skip]
#[derive(Default, Clone)]
pub struct OptStr {
    pub num: i32,           // number
    pub sho: String,        // short option
    pub lon: String,        // long option
    pub meta: String,       // option's meta
    pub _comment: String,   // option comment
    //
    pub meta_type: MetaType,    // meta's type
    pub is_vec: bool,           // Vec<meta type>
    pub enum_s: String,         // enume field string
    pub field_s: String,        // struct field string
}

impl OptStr {
    fn to_enum(&self) -> String {
        let r = &self.lon;
        let v: Vec<_> = r
            .split('-')
            .map(|w| {
                let mut cs: Vec<char> = w.chars().collect();
                cs[0] = cs[0].to_ascii_uppercase();
                let mut s = String::new();
                for c in cs {
                    s.push(if c == '.' { '_' } else { c });
                }
                s
            })
            .collect();
        v.join("")
    }
    fn to_field(&self) -> String {
        let mut s = String::with_capacity(self.lon.len());
        for c in self.lon.chars() {
            #[rustfmt::skip]
            let c = match c { '-' => '_', '.' => '_', _ => c, };
            s.push(c);
        }
        let prefix = if self.meta.is_empty() { "flg_" } else { "opt_" };
        prefix.to_string() + &s
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetaType {
    Bool = 0, String,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    Isize, Usize,
    F32, F64,
}

impl MetaType {
    pub fn as_str(&self) -> &str {
        match self {
            MetaType::Bool => "bool",
            MetaType::String => "String",
            MetaType::I8 => "i8",
            MetaType::I16 => "i16",
            MetaType::I32 => "i32",
            MetaType::I64 => "i64",
            MetaType::I128 => "i128",
            MetaType::U8 => "u8",
            MetaType::U16 => "u16",
            MetaType::U32 => "u32",
            MetaType::U64 => "u64",
            MetaType::U128 => "u128",
            MetaType::Isize => "isize",
            MetaType::Usize => "usize",
            MetaType::F32 => "f32",
            MetaType::F64 => "f64",
        }
    }
}

impl Default for MetaType {
    fn default() -> Self {
        MetaType::Bool
    }
}

pub fn parse_input_file(in_file: &str) -> anyhow::Result<(Vec<OptStr>, Vec<String>)> {
    let mut vec_line: Vec<String> = Vec::new();
    let mut vec_optstr: Vec<OptStr> = Vec::new();
    //
    let re_1 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_2 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +([^ ].*)$").unwrap();
    let re_3 = regex::Regex::new(r"^ +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_4 = regex::Regex::new(r"^ +--([^ ]+) +([^ ].*)$").unwrap();
    //
    let mut v_num = 0;
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() || line == "Options:" {
            // nothing todo
        } else if let Some(caps) = re_1.captures(&line) {
            //  -C, --continue-at <offset>        Resumed transfer offset
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: caps[3].to_string(),
                _comment: caps[4].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_2.captures(&line) {
            //  -q, --disable             Disable .curlrc
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: "".to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_3.captures(&line) {
            //      --data-binary <data>  HTTP POST binary data
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: caps[2].to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_4.captures(&line) {
            //      --digest              Use HTTP Digest Authentication
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: "".to_string(),
                _comment: caps[2].to_string(),
                ..OptStr::default()
            });
        } else {
            eprintln!("LINE ERROR: {}", line);
            unreachable!();
        }
        vec_line.push(line);
    }
    //
    for v in &mut vec_optstr {
        let v_meta_type = if v.meta.is_empty() { MetaType::Bool } else { MetaType::String };
        //
        v.meta_type = v_meta_type;
        v.enum_s = v.to_enum();
        v.field_s = v.to_field();
    }
    //
    Ok((vec_optstr, vec_line))
}

pub fn gen_src_help(vec_optstr: &[OptStr], vec_line: &[String]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
const OPTIONS_TEXT: &str = r""#;
    for line in vec_line {
        sss += &format!("{}\n", line);
    }
    sss += "\";\n";
    //
    sss += r#"
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    {},\n", rec.enum_s);
    }
    sss += "}\n";
    sss += r#"
impl std::convert::From<u8> for CmdOP {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl CmdOP {
    pub const fn to(self) -> OptNum {
        self as OptNum
    }
}
"#;
    //
    let vec_optstr_sorted = {
        let mut target: Vec<&OptStr> = vec_optstr.iter().map(|o| o).collect();
        target.sort_by(|&a, &b| match a.lon.cmp(&b.lon) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match a.sho.cmp(&b.sho) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.num.cmp(&b.num),
            },
        });
        target
    };
    //
    let s = r#"
#[rustfmt::skip]
const OPT_ARY: [Opt;"#;
    sss += &format!("{}{}] = [\n", s, vec_optstr_sorted.len());
    for rec in vec_optstr_sorted.iter() {
        sss += "    Opt { ";
        if rec.sho.is_empty() {
            sss += "sho: 0u8,  ";
        } else {
            sss += &format!("sho: b'{}', ", rec.sho);
        }
        let s = "\"".to_string() + &rec.lon + "\",";
        sss += &format!("lon: {:-17}", s);
        sss += if rec.meta.is_empty() {
            "has: Arg::No,  "
        } else {
            "has: Arg::Yes, "
        };
        sss += &format!("num: CmdOP::{}.to(), ", rec.enum_s);
        sss += "},\n";
    }
    sss += "];\n";
    //
    let mut vec_optstr_sho_idx: Vec<(_, usize)> = vec_optstr_sorted
        .iter()
        .enumerate()
        .filter(|(_, &o)| !o.sho.is_empty())
        .map(|(i, &o)| (&o.sho, i))
        .collect();
    vec_optstr_sho_idx.sort_by(|a, b| a.0.cmp(&b.0));
    //
    let s = r#"
#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);"#;
    sss += &format!("{}{}] = [\n", s, vec_optstr_sho_idx.len());
    for elm in vec_optstr_sho_idx.iter() {
        sss += &format!("(b'{}',{}),", elm.0, elm.1);
    }
    sss += "];\n";
    //
    sss += r#"
#[derive(Debug, Default, PartialEq)]
pub struct CmdOptConf {
    pub opt_program: String,
    //
"#;
    let mut have_help: bool = false;
    let mut have_version: bool = false;
    for rec in vec_optstr.iter() {
        let v_type = if rec.is_vec {
            format!("Vec<{}>", rec.meta_type.as_str())
        } else {
            format!("{}", rec.meta_type.as_str())
        };
        sss += &format!("    pub {}: {},\n", rec.field_s, v_type);
        if rec.enum_s == "Help" {
            have_help = true;
        }
        if rec.enum_s == "Version" {
            have_version = true;
        }
    }
    sss += r#"    //
    pub arg_params: Vec<String>,
}
"#;
    sss += r#"
impl flood_tide::HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {
"#;
    if have_help {
        sss += r#"
        self.flg_help
"#;
    } else {
        sss += r#"
        false
"#;
    }
    sss += r#"
    }
    fn is_version(&self) -> bool {
"#;
    if have_version {
        sss += r#"
        self.flg_version
"#;
    } else {
        sss += r#"
        false
"#;
    }
    sss += r#"
    }
}
"#;
    //
    let mut vec_mt: Vec<MetaType> = Vec::new();
    for rec in vec_optstr.iter() {
        let mt = rec.meta_type;
        if mt == MetaType::Bool {
            continue
        }
        if ! vec_mt.contains(&mt) {
            vec_mt.push(mt);
        }
    }
    vec_mt.sort();
    for mt in vec_mt.iter() {
        let s = match mt {
            MetaType::Bool => "",
            MetaType::String => r#"
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(x) => Ok(x.to_string()),
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::I8 => r#"
fn value_to_i8(nv: &NameVal<'_>) -> Result<i8, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<i8>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::I16 => r#"
fn value_to_i16(nv: &NameVal<'_>) -> Result<i16, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<i16>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::I32 => r#"
fn value_to_i32(nv: &NameVal<'_>) -> Result<i32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<i32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::I64 => r#"
fn value_to_i64(nv: &NameVal<'_>) -> Result<i64, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<i64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::I128 => r#"
fn value_to_i128(nv: &NameVal<'_>) -> Result<i128, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<i128>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::U8 => r#"
fn value_to_u8(nv: &NameVal<'_>) -> Result<u8, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u8>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::U16 => r#"
fn value_to_u16(nv: &NameVal<'_>) -> Result<u16, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u16>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::U32 => r#"
fn value_to_u32(nv: &NameVal<'_>) -> Result<u32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::U64 => r#"
fn value_to_u64(nv: &NameVal<'_>) -> Result<u64, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::U128 => r#"
fn value_to_u128(nv: &NameVal<'_>) -> Result<u128, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u128>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::Isize => r#"
fn value_to_isize(nv: &NameVal<'_>) -> Result<isize, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<isize>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::Usize => r#"
fn value_to_usize(nv: &NameVal<'_>) -> Result<usize, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<usize>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::F32 => r#"
fn value_to_f32(nv: &NameVal<'_>) -> Result<f32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<f32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
            MetaType::F64 => r#"
fn value_to_f64(nv: &NameVal<'_>) -> Result<f64, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<f64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#,
        };
        sss += s;
    };
    //
    Ok(sss)
}

pub fn gen_src_match(vec_optstr: &[OptStr]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    //
    sss += r#"
match CmdOP::from(nv.opt.num) {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    CmdOP::{} => {{\n", rec.enum_s);
        let s = match rec.meta_type {
            MetaType::Bool => "true",
            MetaType::String => "value_to_string(nv)?",
            MetaType::I8 => "value_to_i8(nv)?",
            MetaType::I16 => "value_to_i16(nv)?",
            MetaType::I32 => "value_to_i32(nv)?",
            MetaType::I64 => "value_to_i64(nv)?",
            MetaType::I128 => "value_to_i128(nv)?",
            MetaType::U8 => "value_to_u8(nv)?",
            MetaType::U16 => "value_to_u16(nv)?",
            MetaType::U32 => "value_to_u32(nv)?",
            MetaType::U64 => "value_to_u64(nv)?",
            MetaType::U128 => "value_to_u128(nv)?",
            MetaType::Isize => "value_to_isize(nv)?",
            MetaType::Usize => "value_to_usize(nv)?",
            MetaType::F32 => "value_to_f32(nv)?",
            MetaType::F64 => "value_to_f64(nv)?",
        };
        if !s.is_empty() {
            if rec.is_vec {
                sss += &format!("        conf.{}.push({});\n", rec.field_s, s);
            } else {
                sss += &format!("        conf.{} = {};\n", rec.field_s, s);
            }
        }
        sss += "    }\n";
    }
    sss += r#"}
"#;
    //
    Ok(sss)
}
