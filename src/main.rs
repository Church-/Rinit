extern crate nix;
extern crate signal;

#[macro_use]
extern crate log;

use std::env::{args};
use std::process::Command;
use nix::Error;
use nix::errno::Errno;
use nix::sys::signal::{SIGTERM, SIGINT, SIGCHLD};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::sys::wait::WaitStatus::{Exited, Signaled, StillAlive};
use nix::libc::{c_int};
use log

fn setup_io() -> Result<Ok(),Err> {
	


	Ok()
}

fn reap_processes() {
	let trap = signal::trap::Trap::trap(&[SIGCHLD]);
	for sig in trap {

		match sig {

			SIGCHLD => {

				loop {
					match waitpid(-1, Some(WaitPidFlag::WNOHANG)) {
						Ok(Exited(pid, status)) => {
							continue;
						},
						Ok(Signaled(pid, sig, _)) => {
							continue;
						},	
						Err(Error::sys(Errno:ECHILD)) => {
							break;
						},
						Err(e) => {
							warn!("Error {:?}",e);
						},
					}
				}
			},

			_ => ().
		}
	}
}


fn spawn_services() -> Result<Ok(),Err> {
	//To take in a set of services, possibly from a /etc/initab file to be spawned and then respawned
	//if they died.
	
	
	OK()
}

fn respawn_services() -> Result<Ok(),Err> {
	//To take in a set of services, possibly from a /etc/initab file to be spawned and then respawned
	//if they died.
	
	
	OK()
}



fn main() {
	setup_io().unwrap_or_else(|| panic!("Unable to set up IO"));
	
	loop {
		reap_processes();
		
	}
}
