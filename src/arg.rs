use clap_v3::{App, Arg, ArgMatches};

pub fn get_arg() -> ArgMatches {
    println!("TSV Search  v0.0.1");
    App::new("tsv search")
        .version("0.0.1")
        .author("Yu kaiquan <1962568272@qq.com>")
        .about("Use TSV files to search")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("TSV file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("key")
                .required(true)
                .short('k')
                .long("key")
                .multiple(true) // 允许指定多个键
                .value_name("KEY")
                .help("Search keys (can specify multiple)"),
        )
        .arg(
            Arg::with_name("column")
                .short('c')
                .long("column")
                .value_name("STRING")
                .help("return file column(default:1)")
                .takes_value(true)
                .required(false),
        )
        .get_matches()
}
