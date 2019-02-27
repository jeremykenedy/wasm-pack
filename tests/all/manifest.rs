use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use utils::{self, fixture};
use wasm_pack::{self, license, manifest, readme, Cli};

#[test]
fn it_gets_the_crate_name_default_path() {
    let path = &PathBuf::from(".");
    let crate_data = manifest::CrateData::new(&path).unwrap();
    let name = crate_data.crate_name();
    assert_eq!(name, "wasm_pack");
}

#[test]
fn it_gets_the_crate_name_provided_path() {
    let fixture = fixture::js_hello_world();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    assert_eq!(crate_data.crate_name(), "js_hello_world");
}

#[test]
fn it_checks_has_cdylib_default_path() {
    let fixture = fixture::no_cdylib();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(crate_data.check_crate_config(&step).is_err());
}

#[test]
fn it_checks_has_cdylib_provided_path() {
    let fixture = fixture::js_hello_world();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    crate_data.check_crate_config(&step).unwrap();
}

#[test]
fn it_checks_has_cdylib_wrong_crate_type() {
    let fixture = fixture::bad_cargo_toml();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(crate_data.check_crate_config(&step).is_err());
}

#[test]
fn it_recognizes_a_map_during_depcheck() {
    let fixture = fixture::serde_feature();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    crate_data.check_crate_config(&step).unwrap();
}

#[test]
fn it_creates_a_package_json_default_path() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, "", &step)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");
    assert_eq!(pkg.side_effects, "false");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world.d.ts",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, "", &step)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world.d.ts",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_package_json_provided_path_with_scope() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &Some("test".to_string()), false, "", &step)
        .is_ok());
    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "@test/js-hello-world");
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world.d.ts",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_with_correct_files_on_node() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, "nodejs", &step)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.main, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world_bg.js",
        "js_hello_world.d.ts",
        "js_hello_world.js",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_with_correct_files_on_nomodules() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, "no-modules", &step)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.browser, "js_hello_world.js");
    assert_eq!(pkg.types, "js_hello_world.d.ts");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = [
        "js_hello_world_bg.wasm",
        "js_hello_world.js",
        "js_hello_world.d.ts",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_creates_a_pkg_json_in_out_dir() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("./custom/out");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, false, "", &step)
        .is_ok());

    let package_json_path = &fixture.path.join(&out_dir).join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
}

#[test]
fn it_creates_a_package_json_with_correct_keys_when_types_are_skipped() {
    let fixture = fixture::js_hello_world();
    let out_dir = fixture.path.join("pkg");
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    assert!(crate_data
        .write_package_json(&out_dir, &None, true, "", &step)
        .is_ok());
    let package_json_path = &out_dir.join("package.json");
    fs::metadata(package_json_path).unwrap();
    utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();
    assert_eq!(pkg.name, "js-hello-world");
    assert_eq!(pkg.repository.ty, "git");
    assert_eq!(
        pkg.repository.url,
        "https://github.com/rustwasm/wasm-pack.git"
    );
    assert_eq!(pkg.module, "js_hello_world.js");

    let actual_files: HashSet<String> = pkg.files.into_iter().collect();
    let expected_files: HashSet<String> = ["js_hello_world_bg.wasm", "js_hello_world.js"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
    assert_eq!(actual_files, expected_files);
}

#[test]
fn it_errors_when_wasm_bindgen_is_not_declared() {
    let fixture = fixture::bad_cargo_toml();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    assert!(crate_data.check_crate_config(&step).is_err());
}

#[test]
fn it_does_not_error_when_wasm_bindgen_is_declared() {
    let fixture = fixture::js_hello_world();
    // Ensure that there is a `Cargo.lock`.
    fixture.cargo_check();
    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();
    let step = wasm_pack::progressbar::Step::new(1);
    crate_data.check_crate_config(&step).unwrap();
}

#[test]
fn configure_wasm_bindgen_debug_incorrectly_is_error() {
    let fixture = utils::fixture::Fixture::new();
    fixture.readme().hello_world_src_lib().file(
        "Cargo.toml",
        r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "whatever"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            [package.metadata.wasm-pack.profile.dev.wasm-bindgen]
            debug-js-glue = "not a boolean"
            "#,
    );

    let cli = Cli::from_iter_safe(vec![
        "wasm-pack",
        "build",
        "--dev",
        &fixture.path.display().to_string(),
    ])
    .unwrap();

    let result = fixture.run(cli.cmd);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.iter_chain().any(|c| c
        .to_string()
        .contains("package.metadata.wasm-pack.profile.dev.wasm-bindgen.debug")));
}

#[test]
fn parse_crate_data_returns_unused_keys_in_cargo_toml() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .file(
            "Cargo.toml",
            r#"
            [package]
            authors = ["The wasm-pack developers"]
            description = "so awesome rust+wasm package"
            license = "WTFPL"
            name = "whatever"
            repository = "https://github.com/rustwasm/wasm-pack.git"
            version = "0.1.0"

            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            wasm-bindgen = "0.2"

            # Note: production is not valid.
            [package.metadata.wasm-pack.profile.production.wasm-bindgen]
            debug-js-glue = true
            "#,
        )
        .hello_world_src_lib();

    let result = manifest::CrateData::parse_crate_data(&fixture.path.join("Cargo.toml"));

    assert!(result.is_ok());

    let manifest::ManifestAndUnsedKeys { unused_keys, .. } = result.unwrap();

    assert!(unused_keys.contains("package.metadata.wasm-pack.profile.production"));
}

#[test]
fn it_lists_license_files_in_files_field_of_package_json() {
    let fixture = fixture::dual_license();
    let out_dir = fixture.path.join("pkg");

    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();

    let step = wasm_pack::progressbar::Step::new(3);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    license::copy_from_crate(&crate_data, &fixture.path, &out_dir, &step).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, false, "", &step)
        .unwrap();

    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();

    assert!(
        pkg.files.contains(&"LICENSE-WTFPL".to_string()),
        "LICENSE-WTFPL is not in files: {:?}",
        pkg.files,
    );

    assert!(
        pkg.files.contains(&"LICENSE-MIT".to_string()),
        "LICENSE-MIT is not in files: {:?}",
        pkg.files,
    );
}

#[test]
fn it_lists_readme_in_files_field_of_package_json() {
    let fixture = utils::fixture::Fixture::new();
    fixture
        .readme()
        .hello_world_src_lib()
        .cargo_toml("readme-test-for-package-json");

    let out_dir = fixture.path.join("pkg");

    let crate_data = manifest::CrateData::new(&fixture.path).unwrap();

    let step = wasm_pack::progressbar::Step::new(3);
    wasm_pack::command::utils::create_pkg_dir(&out_dir, &step).unwrap();
    readme::copy_from_crate(&fixture.path, &out_dir, &step).unwrap();
    crate_data
        .write_package_json(&out_dir, &None, false, "", &step)
        .unwrap();

    let package_json_path = &fixture.path.join("pkg").join("package.json");
    fs::metadata(package_json_path).unwrap();
    let pkg = utils::manifest::read_package_json(&fixture.path, &out_dir).unwrap();

    assert!(
        pkg.files.contains(&"README.md".to_string()),
        "README.md is not in files: {:?}",
        pkg.files,
    );
}
