use crate::test_helper::compare_file;
use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("aki-resort")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("aki-cmd/aki-resort-cmd.txt"),
        Some(test_out_path!("aki-resort", "cmd.help.rs.txt")),
        Some(test_out_path!("aki-resort", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "head" => (true, false, MetaType::Usize),
                "tail" => (true, false, MetaType::Usize),
                "according-to" => (
                    false,
                    false,
                    MetaType::Other("opt_according_to_word".into()),
                ),
                "color" => (false, false, MetaType::Other("opt_color_when".into())),
                "max-buffer" => (false, false, MetaType::Other("opt_max_buffer_size".into())),
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
    compare_out_res!("aki-resort", "aki-cmd/aki-resort", "cmd.help.rs.txt");
    compare_out_res!("aki-resort", "aki-cmd/aki-resort", "cmd.match.rs.txt");
}
