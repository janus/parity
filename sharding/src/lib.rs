// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Sharding node library

#![cfg_attr(feature="strict", deny(warnings))]

extern crate futures;
#[macro_use] extern crate log;
extern crate parking_lot;

extern crate ethcore_network as net;
extern crate ethsync;

mod common;
mod message;
mod client;
mod service;
mod chunk;
mod db;

pub use message::{Message, MessagePayload, disconnect as disconnect_message};
pub use common::{ShardId, PeerId, ProtocolVersion, Height, Head, Hash};
pub use client::Client;
pub use service::Service;
pub use chunk::{Body as ChunkBody, Header as ChunkHeader};
pub use db::Database;