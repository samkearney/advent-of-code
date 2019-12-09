
fn password_is_valid(password: &[i32; 6]) -> bool
{
    let mut found_double = false;
    for index in 1..6 {
        if password[index] < password[index - 1] { return false; }
        if password[index] == password[index - 1] { found_double = true; }
    }
    found_double
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_password_is_valid() {
        assert!(password_is_valid(&[1,1,1,1,1,1]));
        assert!(!password_is_valid(&[2,2,3,4,5,0]));
        assert!(!password_is_valid(&[1,2,3,7,8,9]));
    }
}


fn int_to_password_array(mut int_val: i32) -> [i32; 6] {
    let mut arr_val = [0; 6];
    for index in (0..6).rev() {
        arr_val[index] = int_val % 10;
        int_val /= 10;
    }
    arr_val
}

fn main() {
    let mut num_valid_passwords : u32 = 0;

    for password in 264360..746325 {
        if password_is_valid(&int_to_password_array(password)) {
            num_valid_passwords += 1;
        }
    }

    println!("There are {} valid passwords.", num_valid_passwords);
}
