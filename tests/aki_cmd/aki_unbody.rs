use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("aki-unbody")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("aki-cmd/aki-unbody-cmd.txt"),
        Some(test_out_path!("aki-unbody", "cmd.help.rs.txt")),
        Some(test_out_path!("aki-unbody", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "head" => (true, false, MetaType::Usize),
                "tail" => (true, false, MetaType::Usize),
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
    compare_out_res!("aki-unbody", "aki-cmd/aki-unbody", "cmd.help.rs.txt");
    compare_out_res!("aki-unbody", "aki-cmd/aki-unbody", "cmd.match.rs.txt");
}
