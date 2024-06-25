use std::process::Command;

pub fn get_start_command(install_path: &str) -> String {
  if check_java_version(&format!("{}/{}", install_path, "jre64/bin/java")) {
    info!("64-bit java detected");
    build_start_command(install_path, "ProjectZomboid64", "jre64/bin", "linux64", "jre64/lib/amd64")
  } else if check_java_version(&format!("{}/{}", install_path, "jre/bin/java")) {
    info!("32-bit java detected");
    build_start_command(install_path, "ProjectZomboid32", "jre/bin", "linux32", "jre/lib/i386")
  } else {
    panic!("Couldn't determine 32/64 bit of java");
  }
}

fn build_start_command(install_path: &str, exe: &str, java_bin: &str, linux_dir: &str, java_lib: &str) -> String {
  let path = format!("{}/{}:$PATH", install_path, &java_bin);
  let ld_library_path = [
      format!("{}/{}", install_path, linux_dir),
      format!("{}/natives", install_path),
      format!("{}/{}", install_path, java_lib),
      install_path.to_string(),
  ].join(":");

  let exe_path = format!("{}/{}", install_path, exe);

  format!(
    "PATH=\"{}\" LD_LIBRARY_PATH=\"{}\" {}",
    path,
    ld_library_path,
    exe_path)
}

fn check_java_version(java_path: &str) -> bool {
  Command::new(&java_path)
    .arg("-version")
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}
