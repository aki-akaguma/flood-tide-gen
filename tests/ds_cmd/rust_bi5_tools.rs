use crate::test_helper::compare_file;
//
#[test]
fn gen_src_cmd_main() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools")).is_ok());
    //
    let r = do_gen_src_main(
        test_in_path!("ds-cmd/rust-bi5-tools--cmd.txt"),
        test_out_path!("rust-bi5-tools", "cmd.help.rs.txt"),
        test_out_path!("rust-bi5-tools", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!("rust-bi5-tools", "ds-cmd/rust-bi5-tools", "cmd.help.rs.txt");
    compare_out_res!(
        "rust-bi5-tools",
        "ds-cmd/rust-bi5-tools",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_cat() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools-cat")).is_ok());
    //
    let r = do_gen_src_subc(
        test_in_path!("ds-cmd/rust-bi5-tools-cat-cmd.txt"),
        test_out_path!("rust-bi5-tools-cat", "cmd.help.rs.txt"),
        test_out_path!("rust-bi5-tools-cat", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-bi5-tools-cat",
        "ds-cmd/rust-bi5-tools-cat",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-bi5-tools-cat",
        "ds-cmd/rust-bi5-tools-cat",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_conv() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools-conv")).is_ok());
    //
    let r = do_gen_src_subc(
        test_in_path!("ds-cmd/rust-bi5-tools-conv-cmd.txt"),
        test_out_path!("rust-bi5-tools-conv", "cmd.help.rs.txt"),
        test_out_path!("rust-bi5-tools-conv", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-bi5-tools-conv",
        "ds-cmd/rust-bi5-tools-conv",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-bi5-tools-conv",
        "ds-cmd/rust-bi5-tools-conv",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_list() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools-list")).is_ok());
    //
    let r = do_gen_src_subc(
        test_in_path!("ds-cmd/rust-bi5-tools-list-cmd.txt"),
        test_out_path!("rust-bi5-tools-list", "cmd.help.rs.txt"),
        test_out_path!("rust-bi5-tools-list", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-bi5-tools-list",
        "ds-cmd/rust-bi5-tools-list",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-bi5-tools-list",
        "ds-cmd/rust-bi5-tools-list",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_text() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools-text")).is_ok());
    //
    let r = do_gen_src_subc(
        test_in_path!("ds-cmd/rust-bi5-tools-text-cmd.txt"),
        test_out_path!("rust-bi5-tools-text", "cmd.help.rs.txt"),
        test_out_path!("rust-bi5-tools-text", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-bi5-tools-text",
        "ds-cmd/rust-bi5-tools-text",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-bi5-tools-text",
        "ds-cmd/rust-bi5-tools-text",
        "cmd.match.rs.txt"
    );
}

//
use flood_tide_gen::{gen_src_help, gen_src_match, SrcHelpFlags};
use flood_tide_gen::{parse_input_file, update_file};
use flood_tide_gen::{MetaType, OptStr};
//
pub fn do_gen_src_main(in_f: &str, out_f_help: &str, out_f_match: &str) -> anyhow::Result<()> {
    let (mut vec_optstr, vec_line) = parse_input_file(in_f)?;
    //
    fix_type(&mut vec_optstr);
    //
    let sss = gen_src_help(
        &vec_optstr,
        &vec_line,
        SrcHelpFlags {
            cmd_opt_conf_has_subcmd: true,
            ..Default::default()
        },
    )?;
    update_file(&sss, out_f_help)?;
    //
    let sss = gen_src_match(&vec_optstr)?;
    update_file(&sss, out_f_match)?;
    //
    Ok(())
}
pub fn do_gen_src_subc(in_f: &str, out_f_help: &str, out_f_match: &str) -> anyhow::Result<()> {
    let (mut vec_optstr, vec_line) = parse_input_file(in_f)?;
    //
    fix_type(&mut vec_optstr);
    //
    let sss = gen_src_help(
        &vec_optstr,
        &vec_line,
        SrcHelpFlags {
            subcmd_opt_conf: true,
            ..Default::default()
        },
    )?;
    update_file(&sss, out_f_help)?;
    //
    let sss = gen_src_match(&vec_optstr)?;
    update_file(&sss, out_f_match)?;
    //
    Ok(())
}

fn fix_type(vec_optstr: &mut [OptStr]) {
    fn lon_or_sho(v: &OptStr) -> &str {
        if !v.lon.is_empty() {
            v.lon.as_str()
        } else {
            v.sho.as_str()
        }
    }
    //
    for v in vec_optstr {
        let (v_is_opt, v_is_vec, v_meta_type) = match lon_or_sho(v) {
            //"pid" => (false, false, MetaType::I32),
            "otype" => (
                false,
                false,
                MetaType::Other("opt_otype_comp_kind".to_string()),
            ),
            "type" => (false, false, MetaType::Other("opt_type_format".to_string())),
            "from" => (true, false, MetaType::Other("opt_fromto_date".to_string())),
            "to" => (true, false, MetaType::Other("opt_fromto_date".to_string())),
            "digits" => (false, false, MetaType::Usize),
            //
            "X" => (false, true, MetaType::Other("opt_uc_x_param".to_string())),
            _ => (v.is_opt, v.is_vec, v.meta_type.clone()),
        };
        v.is_opt = v_is_opt;
        v.is_vec = v_is_vec;
        v.meta_type = v_meta_type;
    }
}
