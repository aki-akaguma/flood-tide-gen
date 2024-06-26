use anyhow::Context;
use case::CaseExt;
use std::io::{BufRead, Read, Write};

mod gen_buffer;
mod gen_src_help;
mod gen_src_match;

pub(crate) use gen_buffer::GenBuffer;

#[allow(deprecated)]
pub use gen_src_help::gen_src_help;
#[allow(deprecated)]
pub use gen_src_match::gen_src_match;

pub(crate) use gen_src_help::gen_src_help0;
pub(crate) use gen_src_match::gen_src_match0;

#[allow(deprecated, dead_code)]
#[deprecated(
    since = "0.1.10",
    note = "Please use the do_gen_src<F>() function instead"
)]
pub fn update_file(sss: &str, file_path: &str) -> anyhow::Result<()> {
    update_file0(sss, file_path)
}
pub(crate) fn update_file0(sss: &str, file_path: &str) -> anyhow::Result<()> {
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

///
/// Command Option String after parsing
///
#[derive(Debug, Default, Clone)]
pub struct OptStr {
    /// number
    pub num: i32,
    /// short option
    pub sho: String,
    /// long option
    pub lon: String,
    /// option's meta
    pub meta: String,
    /// option comment
    pub comment: String,
    //
    /// meta's type
    pub meta_type: MetaType,
    /// vec type?: Vec<meta type>
    pub is_vec: bool,
    /// option type?: Option<meta type>
    pub is_opt: bool,
    /// enume field string
    pub enum_s: String,
    /// struct field string
    pub field_s: String,
}

impl OptStr {
    /// long or short name
    pub fn lon_or_sho(&self) -> &str {
        if !self.lon.is_empty() {
            self.lon.as_str()
        } else {
            self.sho.as_str()
        }
    }
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
        } else if let Some(c) = self.sho.chars().next() {
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
    fn to_field(&self) -> String {
        let s = if !self.lon.is_empty() {
            let mut s = String::with_capacity(self.lon.len());
            for c in self.lon.chars() {
                #[rustfmt::skip]
                let c = match c { '-' => '_', '.' => '_', _ => c, };
                s.push(c);
            }
            s
        } else if let Some(c) = self.sho.chars().next() {
            if c.is_ascii_lowercase() {
                "lc_".to_string() + &self.sho
            } else if c.is_ascii_uppercase() {
                "uc_".to_string() + &self.sho.to_lowercase()
            } else {
                "cc_".to_string() + &self.sho
            }
        } else {
            "".to_string()
        };
        let prefix = if self.meta.is_empty() { "flg_" } else { "opt_" };
        prefix.to_string() + &s
    }
}

/// Meta Type of command option
#[rustfmt::skip]
#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetaType {
    Bool, String,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    Isize, Usize,
    F32, F64,
    Other(String),
}
impl MetaType {
    /// string of meta type
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

#[allow(deprecated, dead_code)]
#[deprecated(
    since = "0.1.10",
    note = "Please use the do_gen_src<F>() function instead"
)]
pub fn parse_input_file(in_file: &str) -> anyhow::Result<(Vec<OptStr>, Vec<String>)> {
    parse_input_file0(in_file)
}

pub(crate) fn parse_input_file0(in_file: &str) -> anyhow::Result<(Vec<OptStr>, Vec<String>)> {
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
                comment: caps[4].to_string(),
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
                comment: caps[3].to_string(),
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
                comment: caps[3].to_string(),
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
                comment: caps[2].to_string(),
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
                comment: caps[3].to_string(),
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
                comment: caps[2].to_string(),
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

#[deprecated(
    since = "0.1.10",
    note = "Please use the do_gen_src<F>() function instead"
)]
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
#[allow(deprecated)]
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

#[derive(Debug)]
pub(crate) struct SrcHelpFlags0 {
    pub options_text: bool,
    pub cmd_op: bool,
    pub opt_ary: bool,
    pub opt_ary_sho_idx: bool,
    pub cmd_opt_conf: bool,
    pub cmd_opt_conf_has_subcmd: bool,
    pub subcmd_opt_conf: bool,
    pub value_to: bool,
}
impl Default for SrcHelpFlags0 {
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
#[allow(deprecated)]
impl std::convert::From<SrcHelpFlags> for SrcHelpFlags0 {
    fn from(item: SrcHelpFlags) -> SrcHelpFlags0 {
        SrcHelpFlags0 {
            options_text: item.options_text,
            cmd_op: item.cmd_op,
            opt_ary: item.opt_ary,
            opt_ary_sho_idx: item.opt_ary_sho_idx,
            cmd_opt_conf: item.cmd_opt_conf,
            cmd_opt_conf_has_subcmd: item.cmd_opt_conf_has_subcmd,
            subcmd_opt_conf: item.subcmd_opt_conf,
            value_to: item.value_to,
        }
    }
}
