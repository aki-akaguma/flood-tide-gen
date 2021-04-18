use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-execsize")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("gp-cmd-devel/rust-execsize-cmd.txt"),
        Some(test_out_path!("rust-execsize", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-execsize", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "sep-lines" => (false, false, MetaType::U32),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "rust-execsize",
        "gp-cmd-devel/rust-execsize",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-execsize",
        "gp-cmd-devel/rust-execsize",
        "cmd.match.rs.txt"
    );
}
