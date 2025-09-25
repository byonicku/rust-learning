mod counter;
mod point;
mod expense;

use std::collections::HashMap;

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut num_counts: HashMap<i32, i32> = HashMap::new();

    // counting
    for num in nums {
        *num_counts.entry(num).or_insert(0) += 1;
    }

    // highest
    let highest_score = num_counts.values().max().unwrap();
    num_counts.values().filter(|&x| { x == highest_score }).sum()
}

pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;

    for num in nums.clone() {
        min = num.abs().min(min.abs());
    }

    if nums.iter().find(|&x| { x == &min }).is_some() {
        min
    } else {
        -min
    }
}

pub fn added_integer(nums1: Vec<i32>,nums2: Vec<i32>) -> i32 {
    nums2.iter().min().unwrap() - nums1.iter().min().unwrap()
}

pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort();
    nums2.sort();

    let mut hm: HashMap<i32, i32> = HashMap::new();

    for num2 in nums2.clone() {
        for num1 in nums1.clone() {
            *hm.entry(num2 - num1).or_insert(0) += 1;
        }
    }
    let (x, y) = hm.iter().max().unwrap();
    0
}

pub fn compare_version(version1: String, version2: String) -> i32 {
    let vec1 = version1.split(".").collect::<Vec<&str>>();
    let vec2 = version2.split(".").collect::<Vec<&str>>();

    let len1 = vec1.len();
    let len2 = vec2.len();

    let mut i = 0;
    let mut j = 0;

    while i < len1 || j < len2 {
        let cur_value1 = vec1.get(i).unwrap_or(&"0").parse::<i32>().unwrap();
        let cur_value2 = vec2.get(j).unwrap_or(&"0").parse::<i32>().unwrap();

        if cur_value1 > cur_value2  {
            return 1;
        }

        if cur_value2 > cur_value1 {
            return -1;
        }

        i += 1;
        j += 1;
    }

    0
}

pub fn is_valid(s: String) -> bool {
    let mut tmp: Vec<char> = vec![];

    fn is_matching(open: char, close: char) -> bool {
        match open {
            '(' => close == ')',
            '{' => close == '}',
            '[' => close == ']',
            _ => false
        }
    }

    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            tmp.push(ch)
        }

        if ch == ')' || ch == '}' || ch == ']' {
            match tmp.pop() {
                Some(x) => {
                    if !is_matching(x, ch) {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    tmp.is_empty()
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n: usize = nums.len();

    let mut left: usize = 0;
    let mut right: usize = n - 1;

    while left <= right {
        let middle = left + (right - left) / 2;

        if nums[middle] == target {
            return middle as i32;
        } else if nums[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    left as i32
}

pub fn add_item(cart: &mut HashMap<String, i32>, name: &str, qty: i32) {
    *(cart).entry(name.to_string()).or_insert(0) += qty
}

pub fn print_cart(cart: &HashMap<String, i32>) {
    println!("Shopping Cart:");
    for (key, values) in cart {
        println!("{}: {}", key, values);
    }
}

pub fn total_cost(cart: &HashMap<String, i32>, prices: &HashMap<String, i32>) -> i32 {
    let mut tmp = 0;
    print!("Total cost: ");
    for (key, qty) in cart {
        match prices.get(key) {
            Some(price) => {
                print!("{}*{} +", qty, price);
                tmp += qty * price;
            }
            None => {}
        }
    }
    print!(" = {}", tmp);
    tmp
}

fn main() {
    let mut cart: HashMap<String, i32> = HashMap::new();
    let mut price: HashMap<String, i32> = HashMap::new();

    add_item(&mut cart, "Apple", 5);
    add_item(&mut cart, "Banana", 2);
    add_item(&mut cart, "Book", 1);

    add_item(&mut price, "Apple", 3);
    add_item(&mut price, "Banana", 2);
    add_item(&mut price, "Book", 10);

    print_cart(&cart);
    total_cost(&cart, &price);

    add_item(&mut cart, "Apple", 10);

    println!();
    println!();

    print_cart(&cart);
    total_cost(&cart, &price);
}
struct Solution {}

impl Solution {
    pub fn get_value(s: String) -> Vec<i32> {
        let mut count: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            *count.entry(ch).or_insert(0) += 1;
        }

        count.values().cloned().collect()
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let count_s = Self::get_value(s);
        let count_t = Self::get_value(t);

        count_s.iter().all(|element| count_t.contains(element))
    }

    pub fn reverse_str(s: String, k: i32) -> String {
        let chunks = s.chars().into_iter().collect::<Vec<char>>();
        let mut res: Vec<Vec<char>> = vec![];

        let mut i = 0;
        for chunk in chunks.clone().chunks(k as usize) {
            if i % 2 != 0 {
                let str: Vec<_>= chunk.iter().cloned().collect();
                res.push(str);

                i += 1;
                continue;
            }

            let str: Vec<_>= chunk.iter().rev().cloned().collect();
            res.push(str);
            i += 1;
        }

        res.into_iter().flat_map(|x| {x.into_iter()}).collect()
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp: Vec<i32> = Vec::new();

        dp[0] = triangle[0][0];

        for i in 1..n {
            let m = triangle[i].len();

            dp[i] = dp[i - 1] + triangle[i][m - 1];

            for j in (1..=m - 2).rev() {
                dp[j] = dp[j].min(dp[j - 1]) + triangle[i][j];
            }

            dp[0] = dp[0] + triangle[i][0];
        }

        let mut min_path_sum = dp[0];

        for i in 1..n {
            min_path_sum = min_path_sum.min(dp[i]);
        }

        min_path_sum
    }
}