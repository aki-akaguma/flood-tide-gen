use anyhow::Context;
use case::CaseExt;
use std::cmp::Ordering;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

pub fn update_file(sss: &str, file_path: &str) -> anyhow::Result<()> {
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
    pub is_opt: bool,           // Option<meta type>
    pub enum_s: String,         // enume field string
    pub field_s: String,        // struct field string
}

impl OptStr {
    fn to_enum(&self) -> String {
        if !self.lon.is_empty() {
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
        } else {
            if let Some(c) = self.sho.chars().next() {
                if c.is_ascii_lowercase() {
                    "Lc".to_string() + &self.sho.to_uppercase()
                } else if c.is_ascii_uppercase() {
                    "Uc".to_string() + &self.sho
                } else {
                    "Cc".to_string() + &self.sho.to_uppercase()
                }
            } else {
                "".to_string()
            }
        }
    }
    fn to_field(&self) -> String {
        let s = if !self.lon.is_empty() {
            let mut s = String::with_capacity(self.lon.len());
            for c in self.lon.chars() {
                #[rustfmt::skip]
                let c = match c { '-' => '_', '.' => '_', _ => c, };
                s.push(c);
            }
            s
        } else {
            if let Some(c) = self.sho.chars().next() {
                if c.is_ascii_lowercase() {
                    "lc_".to_string() + &self.sho
                } else if c.is_ascii_uppercase() {
                    "uc_".to_string() + &self.sho.to_lowercase()
                } else {
                    "cc_".to_string() + &self.sho
                }
            } else {
                "".to_string()
            }
        };
        let prefix = if self.meta.is_empty() { "flg_" } else { "opt_" };
        prefix.to_string() + &s
    }
}

#[rustfmt::skip]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetaType {
    Bool, String,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    Isize, Usize,
    F32, F64,
    Other(String),
}

impl MetaType {
    pub fn as_type_string(&self) -> String {
        match self {
            MetaType::Bool => "bool".to_string(),
            MetaType::String => "String".to_string(),
            MetaType::I8 => "i8".to_string(),
            MetaType::I16 => "i16".to_string(),
            MetaType::I32 => "i32".to_string(),
            MetaType::I64 => "i64".to_string(),
            MetaType::I128 => "i128".to_string(),
            MetaType::U8 => "u8".to_string(),
            MetaType::U16 => "u16".to_string(),
            MetaType::U32 => "u32".to_string(),
            MetaType::U64 => "u64".to_string(),
            MetaType::U128 => "u128".to_string(),
            MetaType::Isize => "isize".to_string(),
            MetaType::Usize => "usize".to_string(),
            MetaType::F32 => "f32".to_string(),
            MetaType::F64 => "f64".to_string(),
            MetaType::Other(s) => s.to_camel(),
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
    let re_5 = regex::Regex::new(r"^ *-([^ ]) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_6 = regex::Regex::new(r"^ *-([^ ]) +([^ ].*)$").unwrap();
    //
    let mut v_num = 0;
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() || line.ends_with("ptions:") {
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
        } else if let Some(caps) = re_5.captures(&line) {
            //  -C <offset>        Resumed transfer offset
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: "".to_string(),
                meta: caps[2].to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_6.captures(&line) {
            //  -q             Disable .curlrc
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: "".to_string(),
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
        let v_meta_type = if v.meta.is_empty() {
            MetaType::Bool
        } else {
            MetaType::String
        };
        //
        v.meta_type = v_meta_type;
        v.enum_s = v.to_enum();
        v.field_s = v.to_field();
    }
    //
    Ok((vec_optstr, vec_line))
}

pub struct SrcHelpFlags {
    pub options_text: bool,
    pub cmd_op: bool,
    pub opt_ary: bool,
    pub opt_ary_sho_idx: bool,
    pub cmd_opt_conf: bool,
    pub cmd_opt_conf_has_subcmd: bool,
    pub subcmd_opt_conf: bool,
    pub value_to: bool,
}
impl Default for SrcHelpFlags {
    fn default() -> Self {
        Self {
            options_text: true,
            cmd_op: true,
            opt_ary: true,
            opt_ary_sho_idx: true,
            cmd_opt_conf: true,
            cmd_opt_conf_has_subcmd: false,
            subcmd_opt_conf: false,
            value_to: true,
        }
    }
}

pub fn gen_src_help(
    vec_optstr: &[OptStr],
    vec_line: &[String],
    out_flags: SrcHelpFlags,
) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    //
    if !out_flags.options_text {
        sss += "/*\n";
    }
    sss += r#"
const OPTIONS_TEXT: &str = r""#;
    for line in vec_line {
        sss += &format!("{}\n", line);
    }
    sss += "\";\n";
    if !out_flags.options_text {
        sss += "*/\n";
    }
    //
    if !out_flags.cmd_op {
        sss += "/*\n";
    }
    sss += r#"
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOp {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    {},\n", rec.enum_s);
    }
    sss += "}\n";
    sss += r#"
impl std::convert::From<u8> for CmdOp {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl CmdOp {
    pub const fn to(self) -> OptNum {
        self as OptNum
    }
}
"#;
    if !out_flags.cmd_op {
        sss += "*/\n";
    }
    //
    let vec_optstr_sorted = {
        let mut target: Vec<&OptStr> = vec_optstr.iter().collect();
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
    if !out_flags.opt_ary {
        sss += "/*\n";
    }
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
        sss += &format!("num: CmdOp::{}.to(), ", rec.enum_s);
        sss += "},\n";
    }
    sss += "];\n";
    if !out_flags.opt_ary {
        sss += "*/\n";
    }
    //
    let mut vec_optstr_sho_idx: Vec<(_, usize)> = vec_optstr_sorted
        .iter()
        .enumerate()
        .filter(|(_, &o)| !o.sho.is_empty())
        .map(|(i, &o)| (&o.sho, i))
        .collect();
    vec_optstr_sho_idx.sort_by(|a, b| a.0.cmp(&b.0));
    //
    if !out_flags.opt_ary_sho_idx {
        sss += "/*\n";
    }
    let s = r#"
#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);"#;
    sss += &format!("{}{}] = [\n", s, vec_optstr_sho_idx.len());
    for elm in vec_optstr_sho_idx.iter() {
        sss += &format!("(b'{}',{}),", elm.0, elm.1);
    }
    sss += "];\n";
    if !out_flags.opt_ary_sho_idx {
        sss += "*/\n";
    }
    //
    if !out_flags.cmd_opt_conf {
        sss += "/*\n";
    }
    if !out_flags.subcmd_opt_conf {
        sss += r#"
#[derive(Debug, Default, PartialEq)]
pub struct CmdOptConf {
    pub prog_name: String,
"#;
        if out_flags.cmd_opt_conf_has_subcmd {
            sss += r#"    pub subcmd: String,
"#;
        }
    } else {
        sss += r#"
#[derive(Debug, PartialEq)]
pub struct SubCmdOptConf<'a> {
    pub parent: &'a CmdOptConf,
    pub prog_name: String,
"#;
    }
    sss += r#"    //
"#;
    let mut have_help: bool = false;
    let mut have_version: bool = false;
    for rec in vec_optstr.iter() {
        let v_type = if rec.is_vec {
            if rec.is_opt {
                format!("Vec<Option<{}>>", rec.meta_type.as_type_string())
            } else {
                format!("Vec<{}>", rec.meta_type.as_type_string())
            }
        } else if rec.is_opt {
            format!("Option<{}>", rec.meta_type.as_type_string())
        } else {
            rec.meta_type.as_type_string().to_string()
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
    if out_flags.subcmd_opt_conf {
        sss += r#"
impl<'a> SubCmdOptConf<'a> {
    pub fn new(a_parent: &'a CmdOptConf, a_prog_name: String) -> SubCmdOptConf<'a> {
        SubCmdOptConf {
            parent: a_parent,
            prog_name: a_prog_name,
            //
"#;
        for rec in vec_optstr.iter() {
            /*
            let v_init = if rec.is_vec {
                format!("Vec::new()")
            } else {
                if rec.is_opt {
                    format!("None")
                } else {
                    format!("Default::default()")
                }
            };
            */
            let v_init = "Default::default()".to_string();
            sss += &format!("            {}: {},\n", rec.field_s, v_init);
        }
        sss += r#"            //
            arg_params: Vec::new(),
        }
    }
}
"#;
    }
    if out_flags.cmd_opt_conf_has_subcmd {
        sss += r#"
impl flood_tide::SubCommand for CmdOptConf {
    fn set_subcmd(&mut self, a_subcmd: String) {
        self.subcmd = a_subcmd;
    }
}
"#;
    }
    if !out_flags.subcmd_opt_conf {
        sss += r#"
impl flood_tide::HelpVersion for CmdOptConf {
    fn is_help(&self) -> bool {"#;
    } else {
        sss += r#"
impl<'a> flood_tide::HelpVersion for SubCmdOptConf<'a> {
    fn is_help(&self) -> bool {"#;
    }
    if have_help {
        sss += r#"
        self.flg_help"#;
    } else {
        sss += r#"
        false"#;
    }
    sss += r#"
    }
    fn is_version(&self) -> bool {"#;
    if have_version {
        sss += r#"
        self.flg_version"#;
    } else {
        sss += r#"
        false"#;
    }
    sss += r#"
    }
}
"#;
    if !out_flags.cmd_opt_conf {
        sss += "*/\n";
    }
    //
    if !out_flags.value_to {
        sss += "/*\n";
    }
    let mut vec_mt: Vec<&MetaType> = Vec::new();
    for rec in vec_optstr.iter() {
        let mt = &rec.meta_type;
        if let MetaType::Bool = mt {
            continue;
        }
        if !vec_mt.contains(&mt) {
            vec_mt.push(mt);
        }
    }
    vec_mt.sort();
    for mt in vec_mt.iter() {
        let s = match mt {
            MetaType::Bool => "",
            MetaType::String => {
                r#"
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(x) => Ok(x.to_string()),
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}
"#
            }
            MetaType::I8 => {
                r#"
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
"#
            }
            MetaType::I16 => {
                r#"
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
"#
            }
            MetaType::I32 => {
                r#"
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
"#
            }
            MetaType::I64 => {
                r#"
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
"#
            }
            MetaType::I128 => {
                r#"
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
"#
            }
            MetaType::U8 => {
                r#"
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
"#
            }
            MetaType::U16 => {
                r#"
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
"#
            }
            MetaType::U32 => {
                r#"
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
"#
            }
            MetaType::U64 => {
                r#"
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
"#
            }
            MetaType::U128 => {
                r#"
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
"#
            }
            MetaType::Isize => {
                r#"
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
"#
            }
            MetaType::Usize => {
                r#"
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
"#
            }
            MetaType::F32 => {
                r#"
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
"#
            }
            MetaType::F64 => {
                r#"
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
"#
            }
            MetaType::Other(_string) => r#""#,
        };
        sss += s;
    }
    if !out_flags.value_to {
        sss += "*/\n";
    }
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
match CmdOp::from(nv.opt.num) {
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    CmdOp::{} => {{\n", rec.enum_s);
        let s = match &rec.meta_type {
            MetaType::Bool => "true".to_string(),
            MetaType::String => "value_to_string(nv)?".to_string(),
            MetaType::I8 => "value_to_i8(nv)?".to_string(),
            MetaType::I16 => "value_to_i16(nv)?".to_string(),
            MetaType::I32 => "value_to_i32(nv)?".to_string(),
            MetaType::I64 => "value_to_i64(nv)?".to_string(),
            MetaType::I128 => "value_to_i128(nv)?".to_string(),
            MetaType::U8 => "value_to_u8(nv)?".to_string(),
            MetaType::U16 => "value_to_u16(nv)?".to_string(),
            MetaType::U32 => "value_to_u32(nv)?".to_string(),
            MetaType::U64 => "value_to_u64(nv)?".to_string(),
            MetaType::U128 => "value_to_u128(nv)?".to_string(),
            MetaType::Isize => "value_to_isize(nv)?".to_string(),
            MetaType::Usize => "value_to_usize(nv)?".to_string(),
            MetaType::F32 => "value_to_f32(nv)?".to_string(),
            MetaType::F64 => "value_to_f64(nv)?".to_string(),
            MetaType::Other(string) => format!("value_to_{}(nv)?", string),
        };
        if !s.is_empty() {
            if rec.is_vec {
                if rec.is_opt {
                    sss += &format!("        conf.{}.push(Some({}));\n", rec.field_s, s);
                } else {
                    sss += &format!("        conf.{}.push({});\n", rec.field_s, s);
                }
            } else if rec.is_opt {
                sss += &format!("        conf.{} = Some({});\n", rec.field_s, s);
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
