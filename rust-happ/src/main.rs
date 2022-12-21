//! Rust SDK demo host application
//!
// SPDX-License-Identifier: MIT
// Copyright (C) 2022 VTT Technical Research Centre of Finland Ltd

extern crate std;
extern crate happ;

use std::env;
use std::str::from_utf8;
use std::thread;

use happ::{Enclave, Error, Status};
use happ::ocall::{Listener, OCall};
use happ::builder::Builder;
use happ::device::{KEYSTONE_DEVICE_PATH};

/// An application specific message
pub const MSG: &[u8] = "Hello, world!\n".as_bytes();

/// Implementation of OCall Listener
struct Printer {
}

impl Listener for Printer {

    /// A callback that simply prints the ocall payload

    fn on_ocall(&self, ctx: &mut OCall) -> Status {

        let req = ctx.request();

        println!("Enclave: {:#x} {}", req.as_ptr() as u64, req.len());

        let result = from_utf8(req);
        match result {
            Ok(s)  =>  println!("Enclave: {}", s),
            Err(e) => {
                println!("Enclave error: {}", e);
                println!("Data: {:02X?}", req);
            },
        }

        return Status::Success;
    }
}

fn main() {
    // Path to enclave application binary
    let app = env::args().nth(1).unwrap();
    // Path to enclave runtime binary
    let ert = env::args().nth(2).unwrap();

    println!("Building enclave");
    let mut builder = Builder::new();
    /* Setup shared memory bounds */
    if builder.setup_shared_memory(Builder::DEFAULT_UNTRUSTED_PTR,
                                   Builder::DEFAULT_UNTRUSTED_SIZE).is_err() {
        println!("Failed to setup shared memory");
        return;
    }

    /* Setup free memory size */
    if builder.setup_free_memory(Builder::DEFAULT_FREE_MEMORY_SIZE).is_err() {
        println!("Failed to setup free memory");
        return;
    }

    /* Add enclave runtime binary */
    if builder.add(&ert, true).is_err() {
        println!("Failed to add binary: {}", ert);
        return;
    }

    /* Add enclave application binary */
    if builder.add(&app, false).is_err() {
        println!("Failed to add binary: {}", app);
        return;
    }

    /* Create new enclave (opens device) */
    let mut enclave = match Enclave::new(KEYSTONE_DEVICE_PATH) {
        Ok(enc) => enc,
        Err(_) =>  {
            println!("Enclave creation failed");
            return;
        }
    };

    /* Build enclave memory and move it to enclave */
    if let Err(_) = enclave.build(&mut builder) {
        println!("Build failed");
        return;
    }

    println!("Enclave created");

    /* Print enclave hash (not attestation) */
    let mut hash: [u8; 64] = [0; 64];
    if let Ok(hash_len) = enclave.hash(&mut hash) {
        println!("Enclave hash: {:02X?} ({} bytes)",
                 &hash[ .. hash_len], hash_len);
    }

    /* Register ocall listener*/
    let listener = Printer{};
    if let Err(_) = enclave.register_ocall(0x1, &listener) {
        println!("Failed to register ocall listener");
    }

    /* Get handle that can be used to parform ecalls from another thread
     * (borrowing rules)
     */
    let handle = match enclave.handle() {
        Ok(handle) => handle,
        Err(_)     => {
            println!("Couldn't get handle");
            return;
        }
    };

    /* A new thread for making ecalls */
    let handler = thread::spawn(move || {

        /* Attest enclave */
        let nonce: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        println!("Attesting enclave");
        println!("Nonce: {:02X?}", nonce);
        let rv = handle.attest(&nonce);

        match rv {
            Ok(ref evidence) => {
                println!("Attestation report: {:02X?}", evidence.as_bytes());
                /* TODO: Verify evidence */
            },
            Err(error)   => println!("Error: {}", error as u32),
        }

        println!("");
        println!("Making ecall");
        /* Perform an user-defined ecall: */
        let input  = Box::from(MSG);
        let output = handle.ecall(0x1, Some(input));
        match output {
            Ok((status, data)) => {
                println!("Status: {}", status as u32);
                match data {
                    Some(buffer) => {
                        if let Ok(result) = from_utf8(&buffer[..]) {
                            println!("Response: {:02X?}", result);
                        } else {
                            println!("Response: {:02X?}", buffer);
                        }
                    },
                    None         => println!("No data"),
                }
            },
            Err(error) => println!("Error: {}", error as u32),
        }

        println!("");
        println!("Stopping enclave");

        /* Request ecall handler to stop (in enclave)*/
        handle.stop();
    });

    /* Run the enclave, this thread will block */
    let result = enclave.run();
    match result {
        Ok(retval)  => println!("Enclave returned: {}", retval),
        Err(status) => println!("Enclave error: {}", Error::as_usize(status)),
    }

    /* Join with the thread */
    handler.join().unwrap();

    println!("Enclave stopped");
}
