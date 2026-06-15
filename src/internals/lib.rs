#[inline]
pub fn str_or_int(left: &str, right: &str) -> bool {
        left.parse::<i64>().is_ok() && right.parse::<i64>().is_ok()
}

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
