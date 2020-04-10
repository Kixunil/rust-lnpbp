// LNP/BP Rust Library
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use crate::csv::serialize;
use super::*;


/// Marker trait for all RGB-related data types
pub trait RgbData: serialize::Network { }

// TODO: Complete marker trait implementation for all RGB data types
impl RgbData for Transition { }
