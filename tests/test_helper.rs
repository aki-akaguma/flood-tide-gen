//
#[allow(unused_macros)]
macro_rules! test_out_path {
    () => {
        "target/test-out"
    };
    ($a:expr) => {
        concat!(test_out_path!(), "/", $a)
    };
    ($a:expr, $b:expr) => {
        concat!(test_out_path!(), "/", $a, "/", $b)
    };
}
//
#[allow(unused_macros)]
macro_rules! test_in_path {
    () => {
        "fixtures/cmd-txt-in"
    };
    ($a:expr) => {
        concat!(test_in_path!(), "/", $a)
    };
    ($a:expr, $b:expr) => {
        concat!(test_in_path!(), "/", $a, "/", $b)
    };
}
//
#[allow(unused_macros)]
macro_rules! test_res_path {
    () => {
        "fixtures/cmd-txt-res"
    };
    ($a:expr) => {
        concat!(test_res_path!(), "/", $a)
    };
    ($a:expr, $b:expr) => {
        concat!(test_res_path!(), "/", $a, "/", $b)
    };
}
//
#[allow(unused_macros)]
macro_rules! compare_out_res {
    ($a:expr, $b:expr, $c:expr) => {{
        use crate::test_helper::compare_file;

        let r = compare_file(test_out_path!($a, $c), test_res_path!($b, $c));
        match r {
            Err(ref err) => {
                assert_eq!(format!("{:#}", err), "");
            }
            Ok(b) => {
                assert_eq!(b, true);
            }
        }
    }};
}
//
#[allow(dead_code)]
pub fn compare_file<P>(a: P, b: P) -> anyhow::Result<bool>
where
    P: AsRef<std::path::Path> + std::fmt::Display + Copy,
{
    use anyhow::Context;

    let a_ss = std::fs::read_to_string(a).context(format!("{}", a))?;
    let b_ss = std::fs::read_to_string(b).context(format!("{}", b))?;
    let a_ss = a_ss.replace("\r\n", "\n");
    let b_ss = b_ss.replace("\r\n", "\n");
    Ok(a_ss == b_ss)
}
