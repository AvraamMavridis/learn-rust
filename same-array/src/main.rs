fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut c: Vec<i64> = a.clone().into_iter().map(|x| x*x).collect();
    let mut e: Vec<i64> = b.clone().into_iter().map(|x| (x).abs()).collect();

    c.sort();
    e.sort();

    c == e
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

fn main() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);

    let a1 = vec![1, 2, 4, 5, 5, 6, 7, 7];
    let a2 = vec![1, 4, 16, 25, 25, 36, 49, 49, 50];
    testing(a1, a2, false);

}
