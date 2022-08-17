use rand::Rng;

fn main() {
    let sponsors: Vec<&str> = include_str!("./sponsors.txt").split("\n").collect();
    let suffixs: Vec<&str> = include_str!("./suffix.txt").split("\n").collect();

    let sponsor = pick_random(sponsors, 3);
    let suffix = pick_random(suffixs, 1);

    println!(
        "Today our sponsor is:\n{}",
        construct_sponsor(sponsor, suffix)
    );
}

fn pick_random(mut list: Vec<&str>, amount: usize) -> Vec<String> {
    if list.len() <= amount {
        println!("Amount is larger then the size of the array");
        panic!();
    }

    let mut rng = rand::thread_rng();
    let mut to_return: Vec<String> = Vec::new();

    for _i in 0..amount {
        let rand_int = rng.gen_range(0..list.len());
        to_return.push(list[rand_int].replace("\r", ""));
        list.remove(rand_int);
    }

    return to_return;
}

fn construct_sponsor(sponsor: Vec<String>, suffix: Vec<String>) -> String {
    return format!("{} {}", sponsor.join(" "), suffix.join(" "));
}
