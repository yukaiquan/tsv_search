use anyhow::{Error, Result};
use serde_json::Result as SerdeResult;
use serde_json::{from_str, Value};
use std::path::Path;
use textdb::accessor::TsvParse;
use textdb::maps::SafeMemoryMap;
use textdb::{accessor, maps, Table};

pub fn get_key(
    table: &Table<SafeMemoryMap, TsvParse<isize, 0>>,
    keys: &Vec<isize>,
    col: usize,
) -> Result<(), Error> {
    // let result = table.get_matching_lines(key).last();
    // let mut res = "";
    // // 如果查询成功，打印结果
    // if let Some(line) = result {
    //     // println!("{}: {:#?}", key, line.col(col));
    //     res = line.col(col)?.to_string();
    // } else {
    //     println!("you input ID {} not found", key);
    // }
    // 批量查询逻辑
    for key in keys {
        let results = table.get_matching_lines(&key);

        // 获取最后一个匹配项（保持原有逻辑）
        if let Some(line) = results.last() {
            let res = line.col(col);
            println!("[Key: {}] Found: {}", key, print_json(res)?);
        } else {
            println!("[Key: {}] Not found", key);
        }
    }

    Ok(())
}

fn pretty_print_json(json_str: &str) -> SerdeResult<String> {
    let parsed: Value = from_str(json_str)?;
    serde_json::to_string_pretty(&parsed)
}

pub fn print_json(json_str: Result<&str, std::str::Utf8Error>) -> Result<String, Error> {
    let mut res: String = "".to_string();
    match json_str {
        Ok(s) => match pretty_print_json(&s) {
            Ok(pretty) => res = pretty,
            Err(e) => eprintln!("JSON 解析失败: {}", e),
        },
        Err(e) => eprintln!("错误: {}", e),
    }
    Ok(res)
}

pub fn table_is_sorted(table: &Table<SafeMemoryMap, TsvParse<isize, 0>>) -> Result<bool, Error> {
    let sorted = table.is_sorted()?;
    if sorted {
        return Ok(true);
    } else {
        eprintln!("Table is not sorted, skipping...");
        return Ok(false);
    }
}

pub fn read_table(input_file: &str) -> Result<Table<SafeMemoryMap, TsvParse<isize, 0>>, Error> {
    // 判断文件路径是否存在
    let path = Path::new(input_file);

    if path.exists() {
        // println!("路径存在");
    } else {
        eprintln!("you input file {} is exists", input_file);
    }

    // 初始化访问器（假设键是 i64 类型，使用第0列）
    let accessor = accessor::TsvParse::<isize, 0>::default();
    // 加载数据文件
    let map = maps::SafeMemoryMap::from_file(path)?;
    let table = Table::new(map, accessor);

    Ok(table)
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;

    use textdb::accessor;
    use textdb::maps;
    use textdb::Table;
    #[test]
    fn test_read_file() {
        // 初始化访问器（假设键是 i64 类型，使用第0列）
        let accessor = accessor::TsvParse::<isize, 0>::default();
        // 加载数据文件
        let path = Path::new("test_tsvdb.tsv");
        let map = maps::SafeMemoryMap::from_file(path).unwrap();
        let table = Table::new(map, accessor);

        // 确保表是排序的
        // assert!(table.is_sorted().unwrap());
        println!("{}", table_is_sorted(&table).unwrap());
        let user_id_to_search: Vec<isize> = vec![12314, 3124, 6554, 4242];
        let _ = get_key(&table, &user_id_to_search, 1);
        // println!("{:#?}", res);
    }
}
