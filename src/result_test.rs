/*

*/

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num = s.parse::<i32>()?; // 如果 parse 失败，直接返回 Err
    Ok(num)
}

// 等价于

fn parse_number1(s: &str) -> Result<i32, std::num::ParseIntError> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => return Err(e),
    }
}
