use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, OptStr, Pasc};
//
#[test]
fn gen_src_cmd_parent() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-bi5-tools")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Parent,
        test_in_path!("ds-cmd/rust-bi5-tools--cmd.txt"),
        Some(test_out_path!("rust-bi5-tools", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-bi5-tools", "cmd.match.rs.txt")),
        do_fix_type,
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
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("ds-cmd/rust-bi5-tools-cat-cmd.txt"),
        Some(test_out_path!("rust-bi5-tools-cat", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-bi5-tools-cat", "cmd.match.rs.txt")),
        do_fix_type,
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
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("ds-cmd/rust-bi5-tools-conv-cmd.txt"),
        Some(test_out_path!("rust-bi5-tools-conv", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-bi5-tools-conv", "cmd.match.rs.txt")),
        do_fix_type,
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
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("ds-cmd/rust-bi5-tools-list-cmd.txt"),
        Some(test_out_path!("rust-bi5-tools-list", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-bi5-tools-list", "cmd.match.rs.txt")),
        do_fix_type,
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
    let r = do_gen_src(
        Pasc::Subcmd,
        test_in_path!("ds-cmd/rust-bi5-tools-text-cmd.txt"),
        Some(test_out_path!("rust-bi5-tools-text", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-bi5-tools-text", "cmd.match.rs.txt")),
        do_fix_type,
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

pub fn do_fix_type(opt_str: &OptStr) -> Option<FixupType> {
    let tup = match opt_str.lon_or_sho() {
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
