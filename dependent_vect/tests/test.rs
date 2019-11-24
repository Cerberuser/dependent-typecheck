use dependent::*;
use dependent_nat::*;
use dependent_vect::{Vect, vect};
use std::ops::Add;

fn zip_sum<T: Clone + Add<T, Output = T>, N: Nat>(
    first: Vect<T, N>,
    second: Vect<T, N>,
) -> Vect<T, N> {
    first.map_clone(|v1| {
        let v2 = second.freeze();
        v1.iter_mut()
            .zip(v2.iter())
            .for_each(|(i1, i2)| *i1 = i1.clone() + i2.clone());
    })
}

macro_rules! parse_i64_discarding {
    ($string:literal) => {
        vect! {{
            use std::str::FromStr;
            let string: &str = &*$string;
            string.split(",").map(i64::from_str).filter_map(Result::ok)
        }}
    };
}

#[test]
fn summing() {
    let (n1, v1) = vect!(vec![1]);
    let (n2, v2) = vect!(vec![1]);

    assert_eq!(
        NatEq::eq(n1, n2).map(|proof| zip_sum(v1, v2.retag(proof.rev())).into_native()),
        Some(vec![2])
    );
}

#[test]
fn assigning() {
    let (n1, mut v1) = vect!(vec![1]);
    let (n2, v2) = vect!(vec![2]);

    match NatEq::eq(n1, n2) {
        Some(proof) => {
            assert_eq!(v1.into_native(), vec![1]);
            v1 = v2.retag(proof.rev());
            assert_eq!(v1.into_native(), vec![2]);
        }
        None => panic!("Assertion broken - mismatched sizes"),
    };
}

#[test]
fn mismatch() {
    let (n1, _) = vect!(vec![1]);
    let (n2, _) = vect!(vec![2, 3]);
    assert!(NatEq::eq(n1, n2).is_none());
}

#[test]
fn runtime_size() {
    let (n1, v1) = parse_i64_discarding!("1,2,3,four,5");
    let (n2, v2) = parse_i64_discarding!("1,two,3,4,5");
    assert_eq!(
        NatEq::eq(n1, n2).map(|proof| zip_sum(v1, v2.retag(proof.rev())).into_native()),
        Some(vec![2, 5, 7, 10])
    );
}