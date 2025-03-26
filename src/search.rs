use anyhow::{Error, Result};
use serde_json::Result as SerdeResult;
use serde_json::{from_str, Value};
use std::path::Path;
use textdb::accessor::TsvParse;
use textdb::maps::SafeMemoryMap;
use textdb::{accessor, maps, Table};

/// Searches for multiple keys in a table and prints the results.
///
/// This function takes a table, a vector of keys, and a column index. It searches for each key
/// in the table and prints the result. If a key is found, it prints the value from the specified
/// column. If a key is not found, it prints a "Not found" message.
///
/// # Parameters
///
/// * `table` - A reference to the Table to search in, with SafeMemoryMap and TsvParse<isize, 0> as its parameters.
/// * `keys` - A reference to a Vec of isize values representing the keys to search for.
/// * `col` - The index of the column to retrieve the value from when a key is found.
///
/// # Returns
///
/// Returns `Ok(())` if the function completes successfully, or an `Error` if any operation fails.
// pub fn get_key(
//     table: &Table<SafeMemoryMap, TsvParse<isize, 0>>,
//     keys: &Vec<isize>,
//     col: usize,
// ) -> Result<(), Error> {
//     for key in keys {
//         if let Some(line) = table.get_matching_lines(&key).last() {
//             let res = line.col(col);
//             println!("Key: {}\tFound: {}", key, print_json(res)?);
//         } else {
//             println!("Key: {}\tNot found", key);
//         }
//     }
//     Ok(())
// }
pub fn get_key(
    table: &Table<SafeMemoryMap, TsvParse<isize, 0>>,
    keys: &Vec<isize>,
    col: usize,
) -> Result<(), Error> {
    use rayon::prelude::*;   // 引入并行处理库
    use std::sync::Mutex;

    // 错误收集容器（线程安全）
    let errors = Mutex::new(Vec::<Error>::new());

    // 并行处理键查询
    keys.par_iter().for_each(|key| {
        let result = (|| -> Result<(), Error> {
            // 保持不变的部分
            if let Some(line) = table.get_matching_lines(key).last() {
                let res = line.col(col)?;  // 保持原有错误处理逻辑
                println!("Key: {}\tFound: {}", key, print_json(Ok(res))?);
            } else {
                println!("Key: {}\tNot found", key);
            }
            Ok(())
        })();

        // 收集错误到共享容器
        if let Err(e) = result {
            errors.lock().unwrap().push(e);
        }
    });

    // 错误优先返回机制
    let errors = errors.into_inner()?;
    if let Some(first_error) = errors.into_iter().next() {
        Err(first_error)
    } else {
        Ok(())
    }
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
            Err(_) => res = s.to_string(),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(res)
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

#[allow(dead_code)]
fn table_is_sorted(table: &Table<SafeMemoryMap, TsvParse<isize, 0>>) -> Result<bool, Error> {
    let sorted = table.is_sorted()?;
    if sorted {
        return Ok(true);
    } else {
        eprintln!("Table is not sorted, skipping...");
        return Ok(false);
    }
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
        let res = get_key(&table, &user_id_to_search, 1);
        println!("{:#?}", res);
    }
}
