fn sum_with_overflow_check(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in nums {
        if let Some(new_sum) = sum.checked_add(num) {
            sum = new_sum;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    let nums1 = &[1, 2, 3, 4];
    let nums2 = &[u32::MAX, u32::MAX];

    match sum_with_overflow_check(nums1) {
        Some(result) => println!("The sum is {}", result),
        None => println!("The sum overflowed!"),
    }

    match sum_with_overflow_check(nums2) {
        Some(result) => println!("The sum is {}", result),
        None => println!("The sum overflowed!"),
    }
}
