//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // tests7：设置环境变量 TEST_FOO，并让 Cargo 在变化时重新构建
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 告诉 Cargo：如果 TEST_FOO 变了就 rerun
    // println!("cargo:rerun-if-env-changed=TEST_FOO");

    // 设置编译期环境变量 TEST_FOO（可以用 env!("TEST_FOO") 读取）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // tests8：启用条件编译的 cfg，相当于开启 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
