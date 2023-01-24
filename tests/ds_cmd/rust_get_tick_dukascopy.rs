use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("rust-get-tick-dukascopy")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("ds-cmd/rust-get-tick-dukascopy-cmd.txt"),
        Some(test_out_path!("rust-get-tick-dukascopy", "cmd.help.rs.txt")),
        Some(test_out_path!(
            "rust-get-tick-dukascopy",
            "cmd.match.rs.txt"
        )),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "from" => (true, false, MetaType::Other("opt_fromto_date".into())),
                "to" => (true, false, MetaType::Other("opt_fromto_date".into())),
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
        "rust-get-tick-dukascopy",
        "ds-cmd/rust-get-tick-dukascopy",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "rust-get-tick-dukascopy",
        "ds-cmd/rust-get-tick-dukascopy",
        "cmd.match.rs.txt"
    );
}
