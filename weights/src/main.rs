

fn order_weight(s: &str) -> String {
    let mut vec: Vec<&str> = s.split(" ").collect();
    vec.sort();

    let to_int = |x: &str| -> i32 {
        x.split("").into_iter().filter(|x| *x != "").map(|x| x.parse::<i32>().unwrap()).sum()
    };

    vec.sort_by(|a,b| to_int(&a).cmp(&to_int(&b)));
    vec.join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}



fn main() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing("2000 10003 1234000 44444444 9999 11 11 22 123",
            "11 11 2000 10003 22 123 1234000 44444444 9999");
}

