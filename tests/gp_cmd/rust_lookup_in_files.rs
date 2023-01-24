use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-lookup-in-files")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("gp-cmd/rust-lookup-in-files-cmd.txt"),
        Some(test_out_path!("rust-lookup-in-files", "cmd.help.rs.txt")),
        Some(test_out_path!("rust-lookup-in-files", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "sep-lines" => (false, false, MetaType::U32),
                "input" => (true, false, opt_str.meta_type.clone()),
                "output" => (true, false, opt_str.meta_type.clone()),
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
        "rust-lookup-in-files",
        "gp-cmd/rust-lookup-in-files",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-lookup-in-files",
        "gp-cmd/rust-lookup-in-files",
        "cmd.match.rs.txt"
    );
}
