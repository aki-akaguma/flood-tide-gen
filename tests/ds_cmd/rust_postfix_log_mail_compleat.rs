use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-postfix-log-mail-compleat")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("ds-cmd/rust-postfix-log-mail-compleat-cmd.txt"),
        Some(test_out_path!(
            "rust-postfix-log-mail-compleat",
            "cmd.help.rs.txt"
        )),
        Some(test_out_path!(
            "rust-postfix-log-mail-compleat",
            "cmd.match.rs.txt"
        )),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "expire-minutes" => (false, false, MetaType::I64),
                "cap-num1" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num2" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num3" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num4" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num5" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert!(r.is_ok());
    //
    compare_out_res!(
        "rust-postfix-log-mail-compleat",
        "ds-cmd/rust-postfix-log-mail-compleat",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-postfix-log-mail-compleat",
        "ds-cmd/rust-postfix-log-mail-compleat",
        "cmd.match.rs.txt"
    );
}
