use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-iptables-sum")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("ds-cmd/rust-iptables-sum-cmd.txt"),
        Some(test_out_path!("rust-iptables-sum", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-iptables-sum", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "expire-minutes" => (false, false, MetaType::I64),
                "cap-num1" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num2" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
                "cap-num3" => (false, false, MetaType::Other("opt_cap_num_nums".into())),
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
        "rust-iptables-sum",
        "ds-cmd/rust-iptables-sum",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-iptables-sum",
        "ds-cmd/rust-iptables-sum",
        "cmd.match.rs.txt"
    );
}
