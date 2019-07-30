fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rst: Vec<i32> = Vec::new();
    for x in 0..nums.len() {
        //        if nums[x] > target {
        //            continue;
        //        }
        let comp = target - nums[x];
        for (y, v) in nums.iter().enumerate() {
            if y == x {
                continue;
            }

            if comp == *v {
                rst.push(x as i32);
                rst.push(y as i32);
                return rst;
            }
        }
    }
    rst
}

fn main() {
    //    let mut test:Vec<i32>=(0..10).collect();
    let test = vec![-1, -2, -3, -4, -5];
    println!("result {:?}", two_sum(test, -8));
}
