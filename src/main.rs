use meme::arrays;

fn main() {
    let bruh = vec![1, 2, 3, 4];
    let mut sol = arrays::Solution::new(bruh);
    println!("{:?}", sol.pick_index())
}
