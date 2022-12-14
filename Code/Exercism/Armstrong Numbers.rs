pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }
               
    if num < 100 {
        return false;
    }

    let order = order(num);
    let mut total: u64 = 0;
    let mut temp = num;

    while temp != 0 {
        let digit = (temp % 10) as u64;
        total += digit.pow(order);
        temp /= 10;
    }

    return total == num as u64;
}

fn order(mut num: u32) -> u32 {
    let mut count = 0;
    while num != 0 {
        count += 1;
        num /= 10;
    }
    return count;
}
