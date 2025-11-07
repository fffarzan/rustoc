pub fn sample() {
    let nums = vec![1, 2, 3, 4, 5];

    let nums_plus_one: Vec<_> = nums.iter().map(|num| num + 1).collect();
    let nums_filtered: Vec<_> = nums.iter().filter(|&&num| num <= 3).collect();
    let found_three: Option<&i32> = nums.iter().find(|&&num| num == 3);
    let count_nums = nums.iter().count();
    let last_el: Option<&i32> = nums.iter().last(); // the type is Option because of the probablity of nums being an empty vector.
    let min_el: Option<&i32> = nums.iter().min();
    let max_el: Option<&i32> = nums.iter().max();
    let taked_nums: Vec<&i32> = nums.iter().take(3).collect();

    let data: Vec<_> = vec![1, 2, 3, 4, 5].iter().map(|num| num * 3).filter(|num| num > &10).collect();
    for num in data {
        println!("{:?}", num);
    }
}