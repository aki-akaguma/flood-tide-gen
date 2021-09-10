TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.14 (2021-09-10)
=====

* update crates: anyhow(1.0.43)

0.1.13 (2021-05-09)
=====

* update depends: regex(1.5.4)

0.1.12 (2021-04-23)
=====

* change to nv.opt.lon_or_sho(): argument of OptParseError::xxx() 

0.1.11 (2021-04-22)
=====

* add MetaType::Other(string) of gen_src_value_to()

0.1.10 (2021-04-18)
=====

* add fn do_gen_src<>() for more simple
* many deprecated
* refactoring source code

0.1.9 (2021-04-14)
=====

* add test
* separate source code: gen_buffer, gen_src_help, gen_src_match

0.1.8 (2021-04-06)
=====

* add support the single only option: -X <option>

0.1.7 (2021-04-03)
=====

* add cmd_opt_conf_has_subcmd and subcmd_opt_conf into SrcHelpFlags
* change param type: fn update_file(sss: &str, file_path: &str)
* update depends

0.1.6 (2021-03-02)
=====

* add OptStr::is_opt for supporting Option<T> field.

0.1.5 (2021-02-28)
=====

* add MetaType::Other(String)
* change MetaType.as_str() to MetaType.as_type_string()

0.1.4 (2021-02-27)
=====

* add support some "ptions:" line in cmd.txt

0.1.3 (2021-02-21)
=====

* rename enum: CmdOP to CmdOp of output in gen_src_help(), it is clippy friends.
* rename field: opt_program to prog_name of struct CmdOptConf in gen_src_help()

0.1.2 (2021-02-07)
=====

* add out_flags: SrcHelpFlags to gen_src_help().

0.1.1 (2021-02-05)
=====

* add support trait HelpVersion
* modify README.md

0.1.0 (2021-01-17)
=====
first commit
