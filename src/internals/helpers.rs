/// ▄█████ ██████ █████▄        ▄████▄ █████▄        ██ ███  ██ ██████ 
/// ▀▀▀▄▄▄   ██   ██▄▄██▄       ██  ██ ██▄▄██▄       ██ ██ ▀▄██   ██   
/// █████▀   ██   ██   ██ ▄▄▄▄▄ ▀████▀ ██   ██ ▄▄▄▄▄ ██ ██   ██   ██   
/// This function checks if an str is an `i64`, if it is, it returns `true` if it is an int, a
/// false if it is a str.
#[inline]
pub fn str_or_int(left: &str, right: &str) -> bool {
        left.parse::<i64>().is_ok() && right.parse::<i64>().is_ok()
}
                               
/// ███  ██ ▄█████ ██▄  ▄██ █████▄ 
/// ██ ▀▄██ ██     ██ ▀▀ ██ ██▄▄█▀ 
/// ██   ██ ▀█████ ██    ██ ██     
/// This function compares 2 numbers, it requires the two numbers, `right` and `left`, and an
/// operator to check for, if it is a true statement, it returns `true`, if it is false, it returns
/// `false`.
#[inline]
pub fn ncmp(left: i64, right: i64, operator: &str) -> bool {
    match operator {
        "==" => {
            if left == right {
                true
            } else {
                false 
            }
        }
        
        "!=" => {
            if left != right {
                true
            } else {
                false 
            }
        }

        "<=" => {
            if left <= right {
                true
            } else {
                false 
            }
        }

        ">=" => {
            if left >= right {
                true 
            } else {
                false 
    
            }
        }

        _ => false
    }
}
                                             
/// ▄█████ ██████ █████▄  ▄█████ ██▄  ▄██ █████▄ 
/// ▀▀▀▄▄▄   ██   ██▄▄██▄ ██     ██ ▀▀ ██ ██▄▄█▀ 
/// █████▀   ██   ██   ██ ▀█████ ██    ██ ██     
/// This function compares two `str`s, using an operator, if the statement given is true, it returns
/// true, if it is `false`, it returns `false`.
#[inline]
pub fn strcmp(left: &str, right: &str, operator: &str) -> bool {
    match operator {
        "==" => {
            if left == right {
                true
            } else {
                false
            }
        }

        "!=" => {
            if left != right {
                true
            } else {
                false
            }
        }

        _ => false
    }
}
