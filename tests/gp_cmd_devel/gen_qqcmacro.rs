use crate::test_helper::compare_file;
//
#[test]
fn gen_src_cmd() {
    assert!(std::fs::create_dir_all(test_out_path!("gen-qqcmacro")).is_ok());
    //
    let r = do_gen_src(
        test_in_path!("gp-cmd-devel/gen-qqcmacro-cmd.txt"),
        test_out_path!("gen-qqcmacro", "cmd.help.rs.txt"),
        test_out_path!("gen-qqcmacro", "cmd.match.rs.txt"),
    );
    if let Err(ref err) = r {
        assert_eq!(format!("{:#}", err), "");
    }
    assert_eq!(r.is_ok(), true);
    //
    compare_out_res!(
        "gen-qqcmacro",
        "gp-cmd-devel/gen-qqcmacro",
        "cmd.help.rs.txt"
    );
    compare_out_res!(
        "gen-qqcmacro",
        "gp-cmd-devel/gen-qqcmacro",
        "cmd.match.rs.txt"
    );
}

//
use flood_tide_gen::gen_src_match;
use flood_tide_gen::parse_input_file;
use flood_tide_gen::update_file;
use flood_tide_gen::{gen_src_help, SrcHelpFlags};
use flood_tide_gen::{MetaType, OptStr};
//
pub fn do_gen_src(in_f: &str, out_f_help: &str, out_f_match: &str) -> anyhow::Result<()> {
    let (mut vec_optstr, vec_line) = parse_input_file(in_f)?;
    //
    fix_type(&mut vec_optstr);
    //
    let sss = gen_src_help(&vec_optstr, &vec_line, SrcHelpFlags::default())?;
    update_file(&sss, out_f_help)?;
    //
    let sss = gen_src_match(&vec_optstr)?;
    update_file(&sss, out_f_match)?;
    //
    Ok(())
}
//
fn fix_type(vec_optstr: &mut [OptStr]) {
    for v in vec_optstr {
        let v_meta_type = match v.lon.as_str() {
            /*
            "speed-time" => MetaType::U32,
            "tftp-blksize" => MetaType::U32,
            */
            "sep-lines" => MetaType::U32,
            _ => v.meta_type.clone(),
        };
        //
        v.meta_type = v_meta_type;
        //
        /*
        let v_is_vec = match v.lon.as_str() {
            "exp" => true,
            "format" => true,
            _ => false,
        };
        v.is_vec = v_is_vec;
        */
    }
}
