fn goto_workdir(file_path: impl AsRef<std::path::Path>) {
    let file_path = file_path.as_ref();
    let mut path = std::env::current_dir().expect("Current dir");
    loop {
        if file_path.exists() {
            break;
        }
        path = path.parent().expect("Couldnt go up").into();

        std::env::set_current_dir(&path).expect("Couldnt go up");
    }
}

pub fn snap(actual_debug: String, file: &str, test_case_name: &str) {
    println!("{}", actual_debug);

    use std::io::Write;
    use std::path::PathBuf;

    let file_path = PathBuf::from(file);

    goto_workdir(&file_path);

    let mut dir_path = file_path.clone();
    dir_path.set_extension("");
    let file_name = dir_path.file_stem().expect("File_name");

    let mut dir_path: PathBuf = file_path.parent().expect("Parent directory").into();

    dir_path.push("snaps");
    dir_path.push(file_name);

    let path: std::path::PathBuf = dir_path.join(format!("{}.snap", test_case_name));
    let new_path: std::path::PathBuf = dir_path.join(format!("{}.snap.new", test_case_name));

    if !path.exists() {
        let _r = std::fs::create_dir_all(&dir_path);
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
