use flood_tide_gen::{do_gen_src, FixupType, MetaType, Pasc};
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("aki-stats")).is_ok());
    //
    let r = do_gen_src(
        Pasc::Void,
        test_in_path!("aki-cmd/aki-stats-cmd.txt"),
        Some(test_out_path!("aki-stats", "cmd.help.rs.txt")),
        Some(test_out_path!("aki-stats", "cmd.match.rs.txt")),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "head" => (false, false, MetaType::Usize),
                "tail" => (false, false, MetaType::Usize),
                "locale" => (false, false, MetaType::Other("opt_locale_loc".into())),
                "query" => (true, false, opt_str.meta_type.clone()),
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
    compare_out_res!("aki-stats", "aki-cmd/aki-stats", "cmd.help.rs.txt");
    compare_out_res!("aki-stats", "aki-cmd/aki-stats", "cmd.match.rs.txt");
}
