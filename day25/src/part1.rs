use day25::SubjectNumber;

pub mod lib;

fn main() {
    let a_public = 5099500;
    let b_public = 7648211;
    let a_loops = lib::count_required_loops(7, a_public);
    let b_loops = lib::count_required_loops(7, b_public);
    println!("a loops: {:?}, b loops: {:?}", a_loops, b_loops);
    let mut subj = SubjectNumber::new(a_public);
    subj.transform_many(b_loops);
    let mut subj2 = SubjectNumber::new(b_public);
    subj2.transform_many(a_loops);
    println!("Encryption Key: {}|{}", subj.value, subj2.value);
}
