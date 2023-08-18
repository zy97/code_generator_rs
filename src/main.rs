mod args;

fn main() {
    let result = 1;
    println!("Hello, world! {}", result);
}
// fn main_result() -> Result<(), CodeGeneratorError> {
//     let Arguments {
//         input_file,
//         charsets,
//         workers,
//         min_password_len,
//         max_password_len,
//         password_dictionary,
//         custom_chars,
//     } = get_args()?;
//     let mut charsets = if custom_chars.len() > 0 {
//         custom_chars
//     } else {
//         charsets
//     };
//     charsets.sort();
//     charsets.dedup();
//     let strategy = match password_dictionary {
//         Some(dict_path) => {
//             let path = Path::new(&dict_path);
//             Strategy::PasswordFile(path.to_path_buf())
//         }
//         None => Strategy::GenPasswords {
//             charsets,
//             min_password_len,
//             max_password_len,
//         },
//     };

//     let workers = workers.unwrap_or_else(num_cpus::get_physical);
//     println!("Starting {} workers to test passwords", workers);

//     let crack = Arc::new(Cracker::new(input_file, workers, strategy));
//     let count = crack.count()?;
//     let progress_bar = Arc::new(create_progress_bar(count as u64));
//     let progress_bar1 = Arc::clone(&progress_bar);
//     let crack1 = Arc::clone(&crack);
//     thread::spawn(move || loop {
//         thread::sleep(Duration::from_millis(500));
//         progress_bar1.set_position(crack1.tested_count());
//     });
//     match crack.start() {
//         Ok(Some(password)) => {
//             println!("Found password: {}", password);
//         }
//         Ok(None) => {
//             println!("Password not found");
//         }
//         Err(_) => {}
//     };
//     Ok(())
// }
