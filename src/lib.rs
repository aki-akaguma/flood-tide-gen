mod gen;

#[allow(deprecated, dead_code)]
pub use gen::{gen_src_help, gen_src_match, parse_input_file, update_file, SrcHelpFlags};

use gen::{gen_src_help0, gen_src_match0, parse_input_file0, update_file0, SrcHelpFlags0};
pub use gen::{MetaType, OptStr};

///
/// read input file and generate out help source, match source file.
///
/// # Examples
/// ## Simple Example
///
/// ```rust
/// use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
///
/// fn do_gen_cmd() -> anyhow::Result<()> {
///     do_gen_src(Pasc::Void, "xtask/aki-gsub-cmd.txt",
///         Some("src/conf/cmd.help.rs.txt"), Some("src/conf/cmd.match.rs.txt"),
///         |opt_str| {
///             let tup = match opt_str.lon_or_sho() {
///                 "color" => (false, false, MetaType::Other("opt_color_when".into())),
///                 "exp" => (false, true, opt_str.meta_type.clone()),
///                 "format" => (false, true, opt_str.meta_type.clone()),
///                 _ => return None,
///             };
///             Some(FixupType::from_tuple(tup))
///         },
///     )?;
///     Ok(())
/// }
/// ```
///
/// ## Fixup Type Example
///
/// ```rust
/// use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc, OptStr};
///
/// fn do_gen_cmd() -> anyhow::Result<()> {
///     do_gen_src(Pasc::Void, "xtask/aki-gsub-cmd.txt",
///         Some("src/conf/cmd.help.rs.txt"), Some("src/conf/cmd.match.rs.txt"),
///         do_fix_type
///     )?;
///     Ok(())
/// }
/// pub fn do_fix_type(opt_str: &OptStr) -> Option<FixupType> {
///     let tup = match opt_str.lon_or_sho() {
///         "color" => (false, false, MetaType::Other("opt_color_when".into())),
///         "exp" => (false, true, opt_str.meta_type.clone()),
///         "format" => (false, true, opt_str.meta_type.clone()),
///         _ => return None,
///     };
///     Some(FixupType::from_tuple(tup))
/// }
/// ```
///
pub fn do_gen_src<F>(
    pas: Pasc,
    in_f: &str,
    out_f_help: Option<&str>,
    out_f_match: Option<&str>,
    f: F,
) -> anyhow::Result<()>
where
    F: Fn(&OptStr) -> Option<FixupType>,
{
    let (mut vec_optstr, vec_line) = parse_input_file0(in_f)?;
    //
    fix_type_ary(&mut vec_optstr, f);
    //
    let flags = match pas {
        Pasc::Void => SrcHelpFlags0::default(),
        Pasc::Parent => SrcHelpFlags0 {
            cmd_opt_conf_has_subcmd: true,
            ..Default::default()
        },
        Pasc::Subcmd => SrcHelpFlags0 {
            subcmd_opt_conf: true,
            ..Default::default()
        },
        Pasc::PareSubc => SrcHelpFlags0 {
            cmd_opt_conf_has_subcmd: true,
            subcmd_opt_conf: true,
            ..Default::default()
        },
    };
    let sss = gen_src_help0(&vec_optstr, &vec_line, flags);
    if let Some(s_out_f_help) = out_f_help {
        update_file0(&sss, s_out_f_help)?;
    }
    //
    let sss = gen_src_match0(&vec_optstr);
    if let Some(s_out_f_match) = out_f_match {
        update_file0(&sss, s_out_f_match)?;
    }
    //
    Ok(())
}

fn fix_type_ary<F>(vec_optstr: &mut [OptStr], f: F)
where
    F: Fn(&OptStr) -> Option<FixupType>,
{
    //
    for v in vec_optstr {
        if let Some(vv) = f(v) {
            v.is_opt = vv.is_opt;
            v.is_vec = vv.is_vec;
            v.meta_type = vv.meta_type;
        }
    }
}

/// The parent or subcommand
#[derive(Debug, Clone, Copy)]
pub enum Pasc {
    /// not parent and not subcommand
    Void,
    /// parent of subcommands
    Parent,
    /// subcommand
    Subcmd,
    /// parent and subcommand
    PareSubc,
}

/// Fixup Type of field
#[derive(Debug)]
pub struct FixupType {
    is_opt: bool,
    is_vec: bool,
    meta_type: MetaType,
}
impl FixupType {
    pub fn new(is_opt: bool, is_vec: bool, meta_type: MetaType) -> FixupType {
        FixupType {
            is_opt,
            is_vec,
            meta_type,
        }
    }
    pub fn from_tuple(a: (bool, bool, MetaType)) -> FixupType {
        FixupType {
            is_opt: a.0,
            is_vec: a.1,
            meta_type: a.2,
        }
    }
}
/*
pub fn extern_fix_type(opt_str: &OptStr) -> Option<FixupType> {
    let tup = match opt_str.lon_or_sho() {
        //"pid" => (false, false, MetaType::I32),
        "otype" => (false, false, MetaType::Other("opt_otype_comp_kind".into())),
        "type" => (false, false, MetaType::Other("opt_type_format".into())),
        "from" => (true, false, MetaType::Other("opt_fromto_date".into())),
        "to" => (true, false, MetaType::Other("opt_fromto_date".into())),
        "digits" => (false, false, MetaType::Usize),
        //
        "X" => (false, true, MetaType::Other("opt_uc_x_param".into())),
        _ => return None,
    };
    Some(FixupType::from_tuple(tup))
}
*/
