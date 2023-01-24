use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("linux-getopt")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("gp-cmd/linux-getopt-cmd.txt"),
        Some(test_out_path!("linux-getopt", "cmd.help.rs.txt")),
        Some(test_out_path!("linux-getopt", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "longoptions" => (true, false, opt_str.meta_type.clone()),
                "options" => (true, false, opt_str.meta_type.clone()),
                "shell" => (true, false, opt_str.meta_type.clone()),
                //
                "X" => (false, true, MetaType::Other("opt_x_param".into())),
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
    compare_out_res!("linux-getopt", "gp-cmd/linux-getopt", "cmd.help.rs.txt");
    compare_out_res!("linux-getopt", "gp-cmd/linux-getopt", "cmd.match.rs.txt");
}
