extern crate turing;

use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use turing::{ Tape, turing, Dir::* };

fn main() {
    let mut args = ::std::env::args().skip(1);
    let machine = args.next().unwrap_or("copy".to_owned());
    let input = args.next();
    let input = input.as_ref().map(|s|s.as_ref());

    match machine.as_ref() {
        "mul" => unary_mul(input),
        "sqr" => check_sqr(input),
        "copy" => copy(input),
        "calc" => calcuator(input),
        m => println!("Unknown machine: {}", m),
    }
}

fn print_state<Sym: Display, Mem: Display>(tape: &Tape<Sym>, mem: &Mem) {
    println!("mem: {}", mem);
    println!("{}", tape);
    sleep(Duration::from_millis(200));
}

fn print_slices<Sym: Display, Mem: Display, A: AsRef<[Tape<Sym>]>>(tape: &A, mem: &Mem) {
    println!("mem: {}", mem);
    for t in tape.as_ref() {
        println!("{}", t);
    }
    sleep(Duration::from_millis(200));
}

fn unary_mul(input: Option<&str>) {
    let input = input.unwrap_or("#111#1111#");
    let t = turing!(['_','_','_'];"Start";"Done";
        ("Start") {['#','_','_'] => ([Right, Right, Stay], ['#', '#', '_'], "Par1")},
        ("Par1") {
            ['1','_','_'] => ([Right, Right, Stay], ['1', '1', '_'], "Par1")
            ['#','_','_'] => ([Right, Stay, Right], ['#', '#', '#'], "Par2")
        },
        ("Par2") {
            ['1','#','_'] => ([Right, Stay, Right], ['1', '#', '1'], "Par2")
            ['#','#','_'] => ([Left, Stay, Left], ['_', '#', '#'], "Reset2")
        },
        ("Reset2") {
            ['1','#','1'] => ([Left, Stay, Left], ['_', '#', '1'], "Reset2")
            ['#','#','#'] => ([Left, Left, Stay], ['_', '#', '#'], "Reset1")
        },
        ("Reset1") {
            ['1','1','#'] => ([Left, Left, Stay], ['_', '1', '#'], "Reset1")
            ['#','#','#'] => ([Right, Right, Right], ['#', '#', '#'], "Calc")
        },
        ("Calc") {
            ['_','1','1'] => ([Right, Right, Stay], ['1', '1', '1'], "Calc")
            ['_','#','1'] => ([Stay, Left, Stay], ['_', '#', '1'], "Calc:Reset")
            ['_','1','#'] => ([Stay, Right, Stay], ['#', '1', '#'], "Cleanup:Climb1")
            ['_','#','#'] => ([Stay, Stay, Stay], ['#', '#', '#'], "Cleanup:Climb1")
        },
        ("Calc:Reset") {
            ['_','1','1'] => ([Stay, Left, Stay], ['_', '1', '1'], "Calc:Reset")
            ['_','#','1'] => ([Stay, Right, Right], ['_', '#', '1'], "Calc")
        },
        ("Cleanup:Climb1") {
            ['#','1','#'] => ([Stay, Right, Stay], ['#', '1', '#'], "Cleanup:Climb1")
            ['#','#','#'] => ([Stay, Left, Stay], ['#', '_', '#'], "Cleanup:Clear1")
        },
        ("Cleanup:Clear1") {
            ['#','1','#'] => ([Stay, Left, Stay], ['#', '_', '#'], "Cleanup:Clear1")
            ['#','#','#'] => ([Stay, Stay, Left], ['#', '_', '_'], "Cleanup:Clear2")
        },
        ("Cleanup:Clear2") {
            ['#','_','1'] => ([Stay, Stay, Left], ['#', '_', '_'], "Cleanup:Clear2")
            ['#','_','#'] => ([Left, Stay, Stay], ['#', '_', '_'], "Cleanup")
        },
        ("Cleanup") {
            ['1','_','_'] => ([Left, Stay, Stay], ['1', '_', '_'], "Cleanup")
            ['#','_','_'] => ([Stay, Stay, Stay], ['#', '_', '_'], "Done")
        },
    );

    let tape = input.chars().collect();
    let tape2 = "_".chars().collect();
    let tape3 = "_".chars().collect();
    match t.debug([tape, tape2, tape3], print_slices) {
        None => println!("Failed"),
        Some([tape, tape2, tape3]) => {
            println!("Finished as");
            println!("{}", tape);
            println!("{}", tape2);
            println!("{}", tape3);
        }
    }
}

fn check_sqr(input: Option<&str>) {
    let input = input.unwrap_or("1111111111111111");
    let t = turing!(['_','_'];"Start";"Done";
        ("Start") { ['1','_'] => ([Right, Stay], ['1', '_'], "Reset") },
        ("Bump") {
            ['1','1'] => ([Right, Right], ['1', '1'], "Bump")
            ['1','_'] => ([Right, Stay], ['1', '1'], "Reset")
        },
        ("Reset") {
            ['1','1'] => ([Right, Left], ['1', '1'], "Reset")
            ['1','_'] => ([Right, Right], ['1', '_'], "Bump")
            ['_','_'] => ([Stay, Right], ['_', '_'], "Done")
        },
    );

    let tape = input.chars().collect();
    let tape2 = "_".chars().collect();
    match t.debug([tape, tape2], print_slices) {
        None => println!("Failed"),
        Some([tape, tape2]) => {
            println!("Finished as");
            println!("{}", tape);
            println!("{}", tape2);
        }
    }
}

fn copy(input: Option<&str>) {
    let input = input.unwrap_or("_abbaab");
    let t = turing! ('_';0;9;
        (0) { '_' => (Right, '_', 1) },
        (1) {
            'a' => (Right, 'A', 2)
            'b' => (Right, 'B', 4)
            '_' => (Left , '_', 8)
        },
        (2) {
            'a' => (Right, 'a', 2)
            'b' => (Right, 'b', 2)
            '_' => (Right, '_', 3)
        },
        (3) {
            'a' => (Right, 'a', 3)
            'b' => (Right, 'b', 3)
            '_' => (Left , 'a', 6)
        },
        (4) {
            'a' => (Right, 'a', 4)
            'b' => (Right, 'b', 4)
            '_' => (Right, '_', 5)
        },
        (5) {
            'a' => (Right, 'a', 5)
            'b' => (Right, 'b', 5)
            '_' => (Left , 'b', 6)
        },
        (6) {
            'a' => (Left , 'a', 6)
            'b' => (Left , 'b', 6)
            '_' => (Left , '_', 7)
        },
        (7) {
            'a' => (Left , 'a', 7)
            'b' => (Left , 'b', 7)
            'A' => (Right, 'A', 1)
            'B' => (Right, 'B', 1)
        },
        (8) {
            'A' => (Left , 'a', 8)
            'B' => (Left , 'b', 8)
            '_' => (Stay , '_', 9)
        },
    );

    let tape = input.chars().collect();
    match t.debug(tape, print_state) {
        None => println!("Failed"),
        Some(state) => {
            println!("Finished as");
            println!("{}", state)
        }
    }
}

fn calcuator(input: Option<&str>) {
    let input = input.unwrap_or("_uucz1100,0101");
    let t = turing! ('_';"Start";"Done";
        ("Start") { '_' => (Right, '_', "Start2") },
        ("Start2") {
            '0' => (Right, '0', "Start2")
            '1' => (Right, '1', "Start2")
            ',' => (Right, ',', "Start2")
            ' ' => (Right, ' ', "Start2")
            '!' => (Right, '!', "Start2")
            'z' => (Right, 'z', "Start2")
            'u' => (Right, 'u', "Start2")
            'c' => (Right, 'c', "Start2")
            's' => (Right, 's', "Start2")
            '|' => (Right, '|', "Start2")
            '&' => (Right, '&', "Start2")
            '+' => (Right, '+', "Start2")
            '-' => (Right, '-', "Start2")
            '_' => (Left , '_', "Main")
        },
        ("Main") {
            '0' => (Left , '0', "Main")
            '1' => (Left , '1', "Main")
            ',' => (Left , ',', "Main")
            '_' => (Stay , '_', "Done")
            ' ' => (Right, ' ', "Space")
            '!' => (Right, ' ', "Not")
            'z' => (Right, ' ', "Zip")
            'u' => (Right, ',', "Unzip")
            'c' => (Right, ' ', "Copy")
            's' => (Right, ' ', "Swap")
            '|' => (Right, ' ', "Or")
            '&' => (Right, ' ', "And")
            '+' => (Right, ' ', "Add")
        },
        ("Space") {
            '0' => (Right, '0', "Space")
            '1' => (Right, '1', "Space")
            ',' => (Right, ',', "Space")
            '_' => (Left , '_', "Space:_")
        },
        ("Space:_") {
            '0' => (Left, '_', "Space:0")
            '1' => (Left, '_', "Space:1")
            ',' => (Left, '_', "Space:,")
            ' ' => (Left, '_', "Main")
        },
        ("Space:0") {
            '0' => (Left, '0', "Space:0")
            '1' => (Left, '0', "Space:1")
            ',' => (Left, '0', "Space:,")
            ' ' => (Left, '0', "Main")
        },
        ("Space:1") {
            '0' => (Left, '1', "Space:0")
            '1' => (Left, '1', "Space:1")
            ',' => (Left, '1', "Space:,")
            ' ' => (Left, '1', "Main")
        },
        ("Space:,") {
            '0' => (Left, ',', "Space:0")
            '1' => (Left, ',', "Space:1")
            ',' => (Left, ',', "Space:,")
            ' ' => (Left, ',', "Main")
        },
        ("Zip") {
            '0' => (Right, '0', "Zip2")
            '1' => (Right, '1', "Zip2")
        },
        ("Zip2") {
            '0' => (Right, ' ', "Zip2:0")
            '1' => (Right, ' ', "Zip2:1")
            ',' => (Right, ' ', "Main")
        },
        ("Zip2:0") {
            '0' => (Right, '0', "Zip2:0")
            '1' => (Right, '0', "Zip2:1")
            ',' => (Right, '0', "Zip3")
        },
        ("Zip2:1") {
            '0' => (Right, '1', "Zip2:0")
            '1' => (Right, '1', "Zip2:1")
            ',' => (Right, '1', "Zip3")
        },
        ("Zip3") {
            '0' => (Left , ',', "Zip3:0")
            '1' => (Left , ',', "Zip3:1")
        },
        ("Zip3:0") {
            '0' => (Left , '0', "Zip3:0")
            '1' => (Left , '1', "Zip3:0")
            ' ' => (Right, '0', "Zip")
        },
        ("Zip3:1") {
            '0' => (Left , '0', "Zip3:1")
            '1' => (Left , '1', "Zip3:1")
            ' ' => (Right, '1', "Zip")
        },
        ("Unzip") {
            '0' => (Left , ' ', "Unzip:0")
            '1' => (Left , ' ', "Unzip:1")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("Unzip:0") {
            '0' => (Left , '0', "Unzip:0")
            '1' => (Left , '1', "Unzip:0")
            ',' => (Right, '0', "Unzip2")
        },
        ("Unzip:1") {
            '0' => (Left , '0', "Unzip:1")
            '1' => (Left , '1', "Unzip:1")
            ',' => (Right, '1', "Unzip2")
        },
        ("Unzip2") {
            '0' => (Right, ',', "Unzip2:0")
            '1' => (Right, ',', "Unzip2:1")
            ' ' => (Right, ',', "Unzip3")
        },
        ("Unzip2:0") {
            '0' => (Right, '0', "Unzip2:0")
            '1' => (Right, '0', "Unzip2:1")
            ' ' => (Right, '0', "Unzip3")
        },
        ("Unzip2:1") {
            '0' => (Right, '1', "Unzip2:0")
            '1' => (Right, '1', "Unzip2:1")
            ' ' => (Right, '1', "Unzip3")
        },
        ("Unzip3") {
            '0' => (Right, '0', "Unzip")
            '1' => (Right, '1', "Unzip")
        },
        ("Copy") {
            '0' => (Left, '0', "Copy:0")
            '1' => (Left, '1', "Copy:1")
        },
        ("Copy:0") { ' ' => (Right, '0', "Copy2") },
        ("Copy:1") { ' ' => (Right, '1', "Copy2") },
        ("Copy2") {
            '0' => (Right, '0', "Copy3")
            '1' => (Right, '1', "Copy3")
        },
        ("Copy3") {
            '0' => (Right, 'c', "Insert:0")
            '1' => (Right, 'c', "Insert:1")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("Insert:0") {
            '0' => (Right, '0', "Insert:0")
            '1' => (Right, '0', "Insert:1")
            ',' => (Right, '0', "Insert:,")
            '_' => (Left , '0', "Main")
        },
        ("Insert:1") {
            '0' => (Right, '1', "Insert:0")
            '1' => (Right, '1', "Insert:1")
            ',' => (Right, '1', "Insert:,")
            '_' => (Left , '1', "Main")
        },
        ("Insert:,") {
            '0' => (Right, ',', "Insert:0")
            '1' => (Right, ',', "Insert:1")
            ',' => (Right, ',', "Insert:,")
        },
        ("Swap") {
            '0' => (Right, ',', "Swap:0")
            '1' => (Right, ',', "Swap:1")
        },
        ("Swap:0") {
            '0' => (Right, '0', "Swap:0")
            '1' => (Right, '0', "Swap:1")
            ',' => (Right, '0', "Swap2")
        },
        ("Swap:1") {
            '0' => (Right, '1', "Swap:0")
            '1' => (Right, '1', "Swap:1")
            ',' => (Right, '1', "Swap2")
        },
        ("Swap2") {
            '0' => (Left , ',', "Swap2:0")
            '1' => (Left , ',', "Swap2:1")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("Swap2:0") {
            '0' => (Left , '0', "Swap2:0")
            '1' => (Left , '1', "Swap2:0")
            ',' => (Right, '0', "Swap")
        },
        ("Swap2:1") {
            '0' => (Left , '0', "Swap2:1")
            '1' => (Left , '1', "Swap2:1")
            ',' => (Right, '1', "Swap")
        },
        ("Not") {
            '0' => (Right, '1', "Not")
            '1' => (Right, '0', "Not")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("Or") {
            '0' => (Right, ' ', "Or:0")
            '1' => (Right, ' ', "Or:1")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("Or:0") {
            '0' => (Right, '0', "Or")
            '1' => (Right, '1', "Or")
        },
        ("Or:1") {
            '0' => (Right, '1', "Or")
            '1' => (Right, '1', "Or")
        },
        ("And") {
            '0' => (Right, ' ', "And:0")
            '1' => (Right, ' ', "And:1")
            ',' => (Left , ',', "Main")
            '_' => (Left , '_', "Main")
        },
        ("And:0") {
            '0' => (Right, '0', "And")
            '1' => (Right, '0', "And")
        },
        ("And:1") {
            '0' => (Right, '1', "And")
            '1' => (Right, '1', "And")
        },
        ("Add") {
            '0' => (Right, '0', "Add")
            '1' => (Right, '1', "Add")
            ',' => (Left , ',', "Add2")
            '_' => (Left , '_', "Add2")
        },
    );

    let tape = input.chars().collect();
    match t.debug(tape, print_state) {
        None => println!("Failed"),
        Some(state) => {
            println!("Finished as");
            println!("{}", state)
        }
    }
}
