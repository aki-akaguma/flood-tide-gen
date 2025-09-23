#[macro_use]
mod test_helper;

use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};

#[test]
fn gen_src_cmd_input_path_type() {
    assert!(std::fs::create_dir_all(test_out_path!("input-path-test")).is_ok());

    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("dummy-cmd/dummy-cmd-input.txt"),
        Some(test_out_path!("input-path-test", "cmd.help.rs.txt")),
        Some(test_out_path!("input-path-test", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "input-file" => (false, false, MetaType::Path),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert!(r.is_ok());

    compare_out_res!(
        "input-path-test",
        "dummy-cmd/input-path-test",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "input-path-test",
        "dummy-cmd/input-path-test",
        "cmd.match.rs.txt"
    );
}

#[test]
fn gen_src_cmd_output_path_type() {
    assert!(std::fs::create_dir_all(test_out_path!("output-path-test")).is_ok());

    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("dummy-cmd/dummy-cmd-output.txt"),
        Some(test_out_path!("output-path-test", "cmd.help.rs.txt")),
        Some(test_out_path!("output-path-test", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "output-file" => (false, false, MetaType::Path),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert!(r.is_ok());

    compare_out_res!(
        "output-path-test",
        "dummy-cmd/output-path-test",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "output-path-test",
        "dummy-cmd/output-path-test",
        "cmd.match.rs.txt"
    );
}
