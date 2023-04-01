fn overwrite<T: Copy>(input: &mut T, new: &mut T) {
    *input = *new;
}

fn test_var_mut() {
    let mut forever_str: &'static str = "hello";
    {
        let string = String::from("world");
        // overwrite(&mut forever_str, &mut &*string);
    }
    println!("{}", forever_str);
}

pub fn run() {
    test_var_mut();
}
