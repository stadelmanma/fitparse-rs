/// Process the FIT profile excel workook and generate the profile module files.
///
/// The entries in the "Types" sheet name and describe the possible values for any Enum field types.
/// The entries in the "Messages" sheet name and number the fields within a given message and specify
/// the type of a field which can be an enum or a primitive. Primitive types may also have units,
/// scale and offset defined which describe how to transform the primitive integer type into a floating
/// point value
#[cfg(not(feature = "profile_parser"))]
fn main() {
    panic!("process_profile executable requires the `profile_parser` feature. \
           Recompile with `cargo build --features profile_parser`");
}

#[cfg(feature = "profile_parser")]
fn main() {
    use std::env::args;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use fitparser::profile::parse_profile;

    let mut argc = 0;
    for argument in args() {
        if argc > 0 {
            let mut profile_fname = PathBuf::from(argument);
            let profile = parse_profile(&profile_fname).unwrap();

            profile_fname.set_extension("json");
            let j = serde_json::to_string(&profile).unwrap();
            let mut f = File::create(profile_fname).unwrap();
            f.write(j.as_bytes()).unwrap();
        }
        argc += 1;
    }
}
