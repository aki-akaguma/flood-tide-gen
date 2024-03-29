use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("aki-mcolor")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("aki-cmd/aki-mcolor-cmd.txt"),
        Some(test_out_path!("aki-mcolor", "cmd.help.rs.txt")),
        Some(test_out_path!("aki-mcolor", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "tftp-blksize" => (false, true, MetaType::U32),
                //
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
    compare_out_res!("aki-mcolor", "aki-cmd/aki-mcolor", "cmd.help.rs.txt");
    compare_out_res!("aki-mcolor", "aki-cmd/aki-mcolor", "cmd.match.rs.txt");
}
