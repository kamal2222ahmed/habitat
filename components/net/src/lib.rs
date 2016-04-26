// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate log;
extern crate protobuf;
extern crate zmq;

pub mod error;
pub mod server;

pub use self::error::{Error, Result};
pub use self::server::{Supervisor, Supervisable};