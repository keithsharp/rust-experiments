use std::fs::{File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let message = "Hello, World!";

    let mut file = File::create("hello.txt").expect("should be able to create file 'hello.txt'");
    let perm = Permissions::from_mode(0o600);
    file.write(message.as_bytes())
        .expect("should be able to write message to file");
    file.set_permissions(perm)
        .expect("should be able to set permissions on file");
    file.sync_all()
        .expect("should be able to sync metadata and data for file");
}
