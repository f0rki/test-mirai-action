use std::fs;

mod foo;
pub mod unsafe_resize;

fn file_thingy() {
    let file_path = "./whatever";
    println!("In file {}", file_path);

    let contents = if std::path::Path::new(file_path).exists() {
        // can panic if unwr
        fs::read_to_string(file_path).unwrap()
    } else {
        "".to_string()
    };

    println!("With text:\n{contents}");
}

fn env_var_thingy() {
    if let Ok(var) = std::env::var("TEST") {
        if let Ok(i) = var.parse::<i32>() {
            match i {
                0..=10 => println!("small"),
                11..=100 => println!("mid-size"),
                i32::MAX => panic!("no no no"),
                _ => println!("large"),
            }
        }
    }

    let var = std::env::var("TEST2");
    if var.is_ok() {
        let var = var.unwrap();
        println!("{var}");
    }
}

fn main() {
    env_var_thingy();
    file_thingy();

    foo::bar(10);
    // foo::bar(usize::MAX - 5);

    match unsafe_resize::do_the_resize() {
        Ok(_) => {}
        Err(e) => println!("do the resize: {:?}", e),
    }
}
