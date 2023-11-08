fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len() - 1;
    while low <= high {
        // FIXME(Shaohua): overflow
        let mid = (low + high) / 2;
        if nums[mid] < target {
            low = mid + 1;
        } else if nums[mid] > target {
            high = mid - 1;
        } else {
            return Some(mid);
        }
    }
    None
}

fn main() {
    const NUMS: &[i32] = &[-7, -3, -2, -2, -1, 2, 4, 5, 6];
    let pos = binary_search(NUMS, 2);
    println!("pos: {pos:?}");
}
