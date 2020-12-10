mod lib;

fn main() {
    let joints = lib::count_joints(
        lib::load_sorted("day10/input.txt")
    );
    
    println!(
        "1-Joints {}, 3-Joints {}, Multiply: {} ", 
        joints[1], 
        joints[3],
        joints[1] * joints[3]
    );
}
