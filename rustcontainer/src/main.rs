extern crate nix;
extern crate clap;
extern crate libc;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use nix::mount::{MsFlags, mount};
use clap::{App, Arg, SubCommand};
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn setup_rootfs(new_root: &str) {
    // Change the current directory to the new root
    std::env::set_current_dir(new_root).expect("Failed to change directory to new root");

    // Convert Rust string to C-style string for chroot
    let new_root_c = CString::new(new_root).expect("Failed to convert to CString");

    // Now, use chroot to change the root directory
    unsafe {
        if libc::chroot(new_root_c.as_ptr()) != 0 {
            panic!("chroot failed: {}", std::io::Error::last_os_error());
        }
    }

    // Change directory again after chroot to ensure we're at the root
    std::env::set_current_dir("/").expect("Failed to change directory after chroot");

    // Ensure /proc exists in the new root
    fs::create_dir_all("/proc").expect("Failed to create /proc directory");

    // Mount the /proc filesystem
    if !is_proc_mounted() {
        // Now, mount the /proc filesystem
        mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
            None::<&str>
        ).expect("Failed to mount /proc");
    }
}


unsafe fn run_container(cmd: &str, args: Vec<&str>) {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            // Parent process waits for the child to finish.
            waitpid(child, None).expect("Failed to wait on child");
        }
        Ok(ForkResult::Child) => {
            // Convert Rust strings to C-style strings for execvp
            let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
            let c_args: Vec<CString> = args.iter()
                .map(|arg| CString::new(*arg).expect("Failed to convert to CString"))
                .collect();
            let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();

            // Unshare namespaces
            unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS).expect("Failed to unshare");

            // Setup the new filesystem root
            let current_dir = std::env::current_dir().unwrap();
            setup_rootfs(&format!("{}/newroot", current_dir.display()));

            execvp(&c_cmd, &c_args_refs).expect("Failed to execvp");
        }
        Err(err) => eprintln!("Fork failed: {}", err),
    }
}

fn main() {

    let binary_path = "/home/luis/rust/appcontainer/target/";  // adjust this path
    let target_dir = "/home/luis/rust/rustcontainer/target/debug/newroot/bin";  // adjust to your container's `bin` directory

    let matches = App::new("Simple Container CLI")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a command in an isolated container")
                .arg(Arg::with_name("COMMAND").required(true).index(1))
                .arg(Arg::with_name("ARGS").multiple(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploys a file or directory to the container root")
                .arg(Arg::with_name("PATH").required(true).index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let cmd = matches.value_of("COMMAND").unwrap();
        let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
        unsafe { run_container(cmd, args); }
    }  else if let Some(matches) = matches.subcommand_matches("deploy") {
        let path = matches.value_of("PATH").unwrap();
        deploy_container(path);
    }

    let proc_cstring = CString::new("/proc").unwrap();
}

fn deploy_container(path: &str) {
    let destination = "./newroot/bin";

    // For simplicity, copy the app to a new_root directory
    let new_root = Path::new("newroot/bin");
    std::fs::create_dir_all(&new_root).expect("Failed to create new root directories.");
    let deploy_path = new_root.join(Path::new(path).file_name().unwrap());
    std::fs::copy(path, &deploy_path).expect("Failed to deploy the app.");
    println!("Deployed to {:?}", deploy_path);
}

fn is_proc_mounted() -> bool {
    let file = match File::open("/proc/mounts") {
        Ok(f) => f,
        Err(_) => return false,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() > 1 && parts[1] == "/proc" {
                return true;
            }
        }
    }
    false
}

