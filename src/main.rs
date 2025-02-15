mod arg;
mod search;
use crate::arg::get_arg;
use crate::search::{get_key, read_table};
use std::path;
use textdb::accessor;
use textdb::maps;
use textdb::Table;

use anyhow::Result;

fn main() -> Result<()> {
    let args = get_arg();
    let input_file = args
        .value_of("input")
        .expect("input tsv DB file is required");
    // let key = args.value_of("key").expect("key is required");
    // 修改为接受多个键
    let keys: Vec<isize> = args
        .values_of("key")
        .expect("至少需要提供一个键")
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .expect("无法解析键值为数字");
    let column = args.value_of("column").unwrap_or("1");
    let column_num: usize = column.parse().expect("please specify column number");
    let table = read_table(&input_file)?;
    let _ = get_key(&table, &keys, column_num);
    Ok(())
}
