mod arg;
mod search;
use crate::arg::get_arg;
use crate::search::{get_key, read_table};

use anyhow::Result;

/// 这个项目最初是为了优化从tsv文件查询基因表达信息而编写的，需要作者提供uniq并排序的键，并以tab分割，值在key之后，默认返还key后的第一列value，value建议格式化为json
/// Executes the main functionality of the TSV search program.//+
/////+
/// This function parses command-line arguments, reads the input TSV file,//+
/// and performs a search operation based on the provided keys and column number.//+
/////+
/// # Returns//+
/////+
/// Returns a `Result<()>` which is `Ok(())` if the program runs successfully,//+
/// or an error if any operation fails.//+
/////+
/// # Errors//+
/////+
/// This function will return an error if://+
/// - Required command-line arguments are missing//+
/// - The input file cannot be read//+
/// - The provided keys cannot be parsed as integers//+
/// - The column number cannot be parsed as a usize//+
/// - The search operation fails//+
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
