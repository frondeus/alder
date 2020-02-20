pub fn snap(actual_debug: String, file: &str, module: &str, test_case_name: &str) {
    println!("{}", actual_debug);

    use std::io::Write;
    let module = module.replace("::", "_");
    let mut dir_path = std::path::PathBuf::from(file);

    dir_path = dir_path.parent().expect("Parent directory").into();
    dir_path.push("snaps");
    let dir_path_str = dir_path.to_str().unwrap();

    let path: std::path::PathBuf =
        format!("{}/{}_{}.snap", dir_path_str, &module, test_case_name).into();
    let new_path: std::path::PathBuf =
        format!("{}/{}_{}.snap.new", dir_path_str, &module, test_case_name).into();

    if !path.exists() {
        dbg!(&dir_path);
        let _r = std::fs::create_dir_all(&dir_path);
        dbg!(&new_path);
        std::fs::File::create(&new_path)
            .and_then(|mut file| file.write_all(actual_debug.as_bytes()))
            .expect("Couldn't save snap");

        panic!("Couldn't find snap. Created new one");
    } else {
        let expected_debug = std::fs::read_to_string(&path).expect("Couldn't read expected snap");
        if expected_debug != actual_debug {
            let _r = std::fs::create_dir_all(&dir_path);
            std::fs::File::create(&new_path)
                .and_then(|mut file| file.write_all(actual_debug.as_bytes()))
                .expect("Couldn't save snap");

            assert_eq!(expected_debug, actual_debug);
        } else if new_path.exists() {
            std::fs::remove_file(&new_path).expect("Couldn't remove new snap");
        }
    }
}
