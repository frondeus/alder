pub fn snap(actual_debug: String, module: &str, test_case_name: &str) {
    println!("{}", actual_debug);

    use std::io::Write;
    let dir_path: std::path::PathBuf = "tests/snaps".into();
    let path: std::path::PathBuf =
        format!("tests/snaps/{}::{}.snap", &module, test_case_name).into();
    let new_path: std::path::PathBuf =
        format!("tests/snaps/{}::{}.snap.new", &module, test_case_name).into();

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
