mod strings;

fn main() {
    // string to vec<char>
    let mut s = "hello".chars().collect::<Vec<char>>();
    strings::reverse::Solution::reverse_string(&mut s);
    // vec<char> to string
    let s = s.into_iter().collect::<String>();
    println!("{}", s);
}
