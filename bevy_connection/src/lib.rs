//! This crate aims to provide a simple and easy to use API for communication between two or more Bevy applications.
//! Note: This is __not__ a networking library, it is a library for communication between Bevy applications running on the same machine.
//!
//! ## Example: Initiator
//! ```rust
//! use bevy::prelude::*;
//! use bevy_connection::prelude::*;
//!
//! fn main() {
//!     let mut app = App::new();
//!
//!     app.add_plugins(InitiatorPlugin);
//!
//!     app.run();
//! }
//! ```
//!
//! ## Example: Client
//! ```rust
//! use bevy::prelude::*;
//! use bevy_connection::prelude::*;
//!
//! fn main() {
//!     let mut app = App::new();
//!
//!     #[cfg(debug_assertions)]
//!     app.add_plugins(ClientPlugin);
//!
//!     app.run();
//! }
//! ```
mod utils;

pub mod client;
pub mod event;
pub mod initiator;

pub mod prelude {
    pub use super::{client::*, event::*, initiator::*};
    pub use bevy::remote::http::{HostAddress, HostPort};
}
