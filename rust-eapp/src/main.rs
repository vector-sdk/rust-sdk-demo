//! Rust SDK demo enclave application
//!
// SPDX-License-Identifier: MIT
// Copyright (C) 2022 VTT Technical Research Centre of Finland Ltd

#![no_std]
#![no_main]

extern crate eapp;
extern crate alloc;
extern crate core;

use alloc::boxed::Box;

use eapp::eapp_entry;
use eapp::ecall;
use eapp::ocall;
use eapp::Status;
use eapp::attestation;

/// Maximum input length accepted by the enclave application
pub const MAX_INPUT_LENGTH : usize = 32;

/// An application specific message
pub const MSG: &[u8] = "Hello, world!\n".as_bytes();

/// Implementation of ECall Listener
struct CopyListener {
}

impl ecall::Listener for CopyListener {

    /// A callback that responds each ecall with the
    /// payload data of included in the request

    fn on_ecall(&self, ctx: &mut ecall::ECall, req: &[u8]) -> Status {

        let res = ctx.response();
        if req.len() > res.len() {
            return Status::ShortBuffer;
        }

        /* Copy data into response */
        res[0 .. req.len()].copy_from_slice(req);
        ctx.response_length(req.len());

        return Status::Success;
    }
}

/// A function that makes an ocall with given cid and payload
pub fn ocall_buf(cid: u64, buffer: &[u8]) -> Status {
    // Reserve buffer for ocall content
    let mut out: [u8; ocall::OCall::HEADER_SIZE + MAX_INPUT_LENGTH]
        = [0; ocall::OCall::HEADER_SIZE + MAX_INPUT_LENGTH];

    // Prepara the OCall
    let mut ctx = match ocall::OCall::prepare(&mut out) {
        Ok(ctx) => ctx,
        Err(_) => { return Status::InternalError }
    };

    // Copy payload data into the OCall request
    let req = ctx.request();
    req[0 .. buffer.len()].copy_from_slice(buffer);
    ctx.request_length(buffer.len());

    // Make the call
    if let Err(_) = ctx.call(cid, true) {
        return Status::InternalError;
    }

    return Status::Success;
}

/// Entry point of the enclave application
///
/// This will be called after the enclave runtime is initialized
#[eapp_entry]
pub fn my_eapp_entry() -> u64 {

    /* Make an ocall: */
    let msg = Box::new(MSG);
    let status = ocall_buf(0x1, &msg);
    if status != Status::Success {
        return status as u64;
    }

    /* Start a server for ecalls: */
    let mut server = ecall::Server::new();

    /* Maximum expected input size: */
    if !server.max_input_size(ecall::ECall::HEADER_SIZE + MAX_INPUT_LENGTH) {
        return Status::InternalError as u64;
    }

    /* Assure that attestation report can fit to ecall output buffer*/
    if !server.max_output_size(attestation::REPORT_MAX_LENGTH) {
        return Status::InternalError as u64;
    }

    /* Register listener */
    let listener = CopyListener{};
    server.register(0x1, &listener);

    return server.serve() as u64;
}
