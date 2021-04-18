use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, OptStr, Pasc};
//
#[test]
fn gen_src_cmd_parent() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-viewexec")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Parent,
        test_in_path!("gp-cmd-devel/rust-viewexec-cmd.txt"),
        Some(test_out_path!("rust-viewexec", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-viewexec", "cmd.match.rs.txt")),
        do_fix_type,
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-viewexec",
        "gp-cmd-devel/rust-viewexec",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-viewexec",
        "gp-cmd-devel/rust-viewexec",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_info() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-viewexec-info")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("gp-cmd-devel/rust-viewexec-info-cmd.txt"),
        Some(test_out_path!("rust-viewexec-info", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-viewexec-info", "cmd.match.rs.txt")),
        do_fix_type,
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-viewexec-info",
        "gp-cmd-devel/rust-viewexec-info",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-viewexec-info",
        "gp-cmd-devel/rust-viewexec-info",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_size() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-viewexec-size")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("gp-cmd-devel/rust-viewexec-size-cmd.txt"),
        Some(test_out_path!("rust-viewexec-size", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-viewexec-size", "cmd.match.rs.txt")),
        do_fix_type,
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-viewexec-size",
        "gp-cmd-devel/rust-viewexec-size",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-viewexec-size",
        "gp-cmd-devel/rust-viewexec-size",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_string() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-viewexec-string")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("gp-cmd-devel/rust-viewexec-string-cmd.txt"),
        Some(test_out_path!("rust-viewexec-string", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-viewexec-string", "cmd.match.rs.txt")),
        do_fix_type,
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-viewexec-string",
        "gp-cmd-devel/rust-viewexec-string",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-viewexec-string",
        "gp-cmd-devel/rust-viewexec-string",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_sc_symbol() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-viewexec-symbol")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("gp-cmd-devel/rust-viewexec-symbol-cmd.txt"),
        Some(test_out_path!("rust-viewexec-symbol", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-viewexec-symbol", "cmd.match.rs.txt")),
        do_fix_type,
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-viewexec-symbol",
        "gp-cmd-devel/rust-viewexec-symbol",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-viewexec-symbol",
        "gp-cmd-devel/rust-viewexec-symbol",
        "cmd.match.rs.txt"
    );
}

pub fn do_fix_type(opt_str: &OptStr) -> Option<FixupType> {
    let tup = match opt_str.lon_or_sho() {
        "sort" => (false, false, MetaType::U8),
        "radix" => (false, false, MetaType::Other("radix_style".into())),
        "format" => (false, false, MetaType::Other("format_style".into())),
        _ => return None,
    };
    Some(FixupType::from_tuple(tup))
}
