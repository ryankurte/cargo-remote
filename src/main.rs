
use std::process::Command;
use std::io;
use std::io::Write;

#[macro_use]
extern crate structopt;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
struct Config {
	#[structopt(long = "remote", short = "r", default_value="192.168.1.152")]
	/// Remote host IP for compilation
	host: String,

	#[structopt(long = "user", short = "u")]
	/// User for remote machine
	user: Option<String>,

	#[structopt(long = "destination", short = "d", default_value="~/.cargo-remote")]
	/// Remote host IP for compilation
	destination: String,

	#[structopt(long = "env", short = "e", default_value="~/.cargo/env")]
	/// Location of cargo env file
	env: String,
	
	#[structopt(long = "target", short = "t", default_value="x86_64-unknown-linux-gnu")]
	/// Toolchain for remote use
	target: String,

	#[structopt(long = "command", short = "c", default_value="build", value_name="COMMAND")]
	/// Command to execute on the remote host
	command: String,
}

impl Config {
	/// Generate a remote URI for use in commands
	pub fn remote(&self) -> String {
		let host = self.host.clone();

		match &self.user {
			Some(user) => format!("{}@{}", user, host),
			None => format!("{}", host),
		}
	}
}

fn push(folder: &str, target: &str) -> Result<(), ()> {
	println!("Pushing source files from: '{}' to: '{}'", folder, target);

	let output = Command::new("rsync")
		.arg("--exclude")
		.arg("target")
		.arg("-rz")
		.args(&[folder, target])
		.output()
		.expect("Failed to execute rsync push");

	if output.status.success() {
		Ok(())
	} else {
		io::stdout().write_all(&output.stdout).unwrap();
		io::stderr().write_all(&output.stderr).unwrap();

		Err(())
	}
}

fn pull(target: &str, folder: &str) -> Result<(), ()> {
	println!("Pulling build files from: '{}' to: '{}'", target, folder);

	let output = Command::new("rsync")
		.arg("-rz")
		.args(&[target, folder])
		.output()
		.expect("Failed to execute rsync pull");

	if output.status.success() {
		Ok(())
	} else {
		io::stdout().write_all(&output.stdout).unwrap();
		io::stderr().write_all(&output.stderr).unwrap();

		Err(())
	}
}

fn exec(target: &str, cmd: &str) -> Result<(), ()> {
	println!("Executing command '{}' on: '{}'", cmd, target);

	let output = Command::new("ssh")
		.args(&[target, cmd])
		.output()
		.expect("Failed to execute cmd");

	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();

	if output.status.success() {
		Ok(())
	} else {
		Err(())
	}
}

fn main() {

	// Load environment
	//let target_dir = env!("OUT_DIR");
	let package_name = env!("CARGO_PKG_NAME");
	let package_version = env!("CARGO_PKG_VERSION");

	let args = std::env::args();
	println!("args: '{:?}'", args);

	// Load options
	let config = Config::from_iter(args.into_iter().filter(|v| v != "remote" ));

	let remote_dir = format!("{}/{}@{}", config.destination, package_name, package_version);
	let remote_path = format!("{}:{}", config.remote(), remote_dir);

	println!("Using remote: '{}'", remote_path);

	// Create directory on target
	exec(&config.remote(), &format!("mkdir -p {}/target", remote_dir))
		.expect("Error creating target directory");

	// Install toolchain on remote
	// TODO: autodetect host toolchain
	exec(&config.remote(), &format!("source {} && rustup target add {}", 
			config.env, config.target))
		.expect("Error installing required toolchain");

	// RSync source files to remote
	push("./", &remote_path)
		.expect("Error pushing source files");

	// Execute command on remote
	exec(&config.remote(), &format!("source {} && cd {} && cargo {} --target={}", 
			config.env, remote_dir, config.command, config.target))
		.expect("Error building target");

	// Ensure local target dir exists
	// TODO: use rust instead of mkdir
	Command::new("mkdir").args(&["-p", &format!("./target/{}", config.target)])
		.output()
		.expect("Failed to create local directory");

	// Copy source files back
	pull(&format!("{}/target/{}/*", remote_path, config.target), &format!("./target/{}", config.target))
		.expect("Error pulling build files");
    
}
