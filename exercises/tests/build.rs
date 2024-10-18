use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // In tests7, we set up an environment variable called `TEST_FOO`.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs(); 
    
    // Set the TEST_FOO environment variable with the current timestamp.
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we enable the "pass" feature.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
