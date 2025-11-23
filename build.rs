use std::{fs, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_protos()?;
    generate_level_xp();
    generate_mnstr_xp();
    Ok(())
}

fn generate_protos() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "adventure_proto/user.proto",
                "adventure_proto/session.proto",
                "adventure_proto/chat.proto",
            ],
            &["adventure_proto"],
        )?;
    Ok(())
}

fn generate_level_xp() {
    let mut levels = vec![];
    levels.push(0);
    levels.push(100);
    for i in 2..101 {
        let previous_xp = levels[i - 1];
        let xp = (previous_xp + ((previous_xp as f64).log10() * 100.0).ceil() as i32) as i32;
        levels.push(xp);
    }
    let ouput = format!("pub const XP_FOR_LEVEL: [i32; 101] = {:?};", levels);
    let mut file = fs::File::create("src/models/generated/level_xp.rs").unwrap();
    file.write_all(ouput.as_bytes()).unwrap();
}

fn generate_mnstr_xp() {
    let mut levels = vec![];
    levels.push(50);
    for i in 1..101 {
        let previous_xp = levels[i - 1];
        let xp = (previous_xp + ((previous_xp as f64).log10() * 10.0).ceil() as i32) as i32;
        levels.push(xp);
    }
    let ouput = format!("pub const XP_FOR_LEVEL: [i32; 101] = {:?};", levels);
    let mut file = fs::File::create("src/models/generated/mnstr_xp.rs").unwrap();
    file.write_all(ouput.as_bytes()).unwrap();
}
