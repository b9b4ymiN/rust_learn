//use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

// ไม่ใช้ semicolon เป็นการ return แบบ expression
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// return หลายค่า แบบ tuple
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    println!("Hello, 🦀");
    // 5 และ 10 เราเรียกมันว่า arguemnts.
    // ส่วน x และ y ใน function add เราเรียก parameter.
    let sum: i32 = add(5, 10);
    println!("sum = {0}", sum);

    let _my_tuples: (i32, i32) = (1, 2);

    // แบบ desstructuring
    let (_first, _second) = (1, 2);
    let swap: (i32, i32) = swap(5, 5555);
    println!("swap = {}", swap.0);

    // Function
    let mut x: i32 = 0;
    let v: &str = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);

    // Using a static method to create an instance of String
    let s: String = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());

    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );

    // concise but assumptive and gets ugly fast
    let v21: f32 = do_something_that_might_fail(42).unwrap();
    println!("found do something = {}", v21);

    Ok(())
}

pub fn create_keypair() -> Keypair {
    Keypair::new()
}
