
fn main(){
    let x = "string";
    let y = "stringa";
    let z = "str";
    let result = compare_string(x, y);

    if result{
        println!("'{}' > '{}'", x, y);
    } else {
        println!("'{}' <= '{}'", x, y);
    }

    let result = compare_string(y, z);

    if result{
        println!("'{}' > '{}'", y, z);
    } else{
        println!("'{}' <= '{}'", y, z);
    }
}

fn compare_string(x: &str, y: &str) -> bool{
    let mut x_chars = x.chars();
    let mut y_chars = y.chars();

    loop{
        match (x_chars.next(), y_chars.next()){
            (Some(x_char), Some(y_char)) => { 
                if x_char > y_char {
                    return true;
                } else if x_char < y_char {
                    return false;
                }
                // 如果当前字符相等，继续比较下一个字符
            }
            (Some(_), None) => {
                // y 较短，但 x 还有字符，所以 x 较大
                return true;
            }
            (None, Some(_)) => {
                // x 较短，但 y 还有字符，所以 y 较大
                return false;
            }
            (None, None) => {
                // 达到字符串末尾，两个字符串相等
                return false;
            }
        }
    }
}
