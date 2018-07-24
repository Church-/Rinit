extern crate nix;
extern crate signal;

#[macro_use]
extern crate log;

use std::env::{args};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::collections::HashMap;
use nix::Error;
use nix::errno::Errno;
use nix::sys::signal::{SIGTERM, SIGINT, SIGCHLD};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::sys::wait::WaitStatus::{Exited, Signaled, StillAlive};
use nix::libc::{c_int};
use log


fn setup_io(daemon_map: HashMap<&str,isize>) -> Result<Ok(),Err> {
	let prod_id = Command::new("agetty1")
			.spawn()
			.args(&[])?.id();

	daemon_map.entry("agetty1").or_insert(prod_id as isize);

	Ok()
}

fn reap_processes(daemon_map: HashMap<&str,isize>) {
	let trap = signal::trap::Trap::trap(&[SIGCHLD]);
	for sig in trap {

		match sig {

			SIGCHLD => {

				loop {
					match waitpid(-1, Some(WaitPidFlag::WNOHANG)) {
						Ok(Exited(pid, status)) => {
							for (key,value) in daemon_map.iter() {
								match v == pid {
									true => daemon_map.remove(&k),
									false => continue,
								}
							}
							continue;
						},
						Ok(Signaled(pid, sig, _)) => {
							for (key,value) in daemon_map.iter() {
								match v == pid {
									true => daemon_map.remove(&k),
									false => continue,
								}
							}	
							continue;
						},	
						Err(Error::sys(Errno:ECHILD)) => {
							break;
						},
						Err(e) => {
							warn!("Error {:?}",e);
							break;
						},
					}
				}
			},

			_ => ().
		}
	}
}


fn spawn_services(daemon_map: HashMap<&str,isize>) -> Result<Ok(),Err> {
	
	
	
	OK()
}

fn respawn_services(daemon_map: HashMap<&str,isize>) -> Result<Ok(),Err> {
	//To take in a set of services, possibly from a /etc/initab file to be spawned and then respawned
	//if they died.
	
	
	OK()
}


fn mount_virt_fs() -> Result<Ok(),Err> {
	let opt_vec: Vec<&str> = vec!["-t proc proc /proc -o nosuid,noexec,nodev",
	"-t sysfs sys /sys -o nosuid,noexec,nodev",
	"-t tmpfs run /run -o mode=0755,nosuid,nodev",
	"-t devtmpfs dev /dev -o mode=0755,nosuid",
	"-t devpts devpts /dev/pts -o mode=0620,gid=5,nosuid,noexec",
	"-t tmpfs shm /dev/shm -o mode=1777,nosuid,nodev"];
	
	for i in opt_vec.iter() {
		let out = 
				Command::new(mount)
				.args(&i.split(" ").collect())
				.output()?;
		println!("Mounting {}",out);
		
	}
	Ok()
}

fn set_hostname() -> Result<Ok(),Err> {
	let mut hostname = String::new();
	let f = File::open("")?;
	f.read_to_string(&mut hostname);
	let mut proc_hostname = File::open()?;
	proc_hostname.write_all(hostname.as_bytes())?;	
	println!("Setting Hostname...");
	Ok()
}

fn init(daemon_map: HashMap<&str, isize> ) -> Result<Ok(),Err> {
	setup_io(daemon_map)?
	mount_virt_fs()?;
	set_hostname()?;
	spawn_services(daemon_map)?;

	Ok()
}


fn main() {
	let mut daemon_map: HashMap<&str, isize> = HashMap::new();
	init(daemon_map);
	loop {
		reap_processes(daemon_map;
		respawn_processes(daemon_map);
	}
}