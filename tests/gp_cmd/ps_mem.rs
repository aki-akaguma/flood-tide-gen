use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("ps-mem")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("gp-cmd/ps-mem-cmd.txt"),
        Some(test_out_path!("ps-mem", "cmd.help.rs.txt")),
        Some(test_out_path!("ps-mem", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "pid" => (false, false, MetaType::I32),
                "sleep" => (false, false, MetaType::U32),
                "sort" => (false, false, MetaType::Other("opt_sort_order".into())),
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
    compare_out_res!("ps-mem", "gp-cmd/ps-mem", "cmd.help.rs.txt");
    compare_out_res!("ps-mem", "gp-cmd/ps-mem", "cmd.match.rs.txt");
}
