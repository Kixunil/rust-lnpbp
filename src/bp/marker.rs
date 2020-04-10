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


use bitcoin::{Block, Transaction, TxIn, TxOut, OutPoint, Script};
use bitcoin::hash_types::*;

use super::scripts::*;


/// Marker trait for all Bitcoin consensus-related and consensus-serializable
/// data structures
pub trait BitcoinData: bitcoin::consensus::encode::Encodable + bitcoin::consensus::encode::Decodable {}

impl BitcoinData for Block { }
impl BitcoinData for Transaction { }
impl BitcoinData for TxIn { }
impl BitcoinData for TxOut { }
impl BitcoinData for OutPoint { }

impl BitcoinData for BlockHash { }
impl BitcoinData for Txid { }
impl BitcoinData for Wtxid { }
impl BitcoinData for SigHash { }
impl BitcoinData for TxMerkleNode { }
impl BitcoinData for WitnessMerkleNode { }
impl BitcoinData for FilterHash { }

/// Marker trait for all possible Bitcoin script variants
pub trait ScriptData { }

impl ScriptData for Script { }
impl ScriptData for LockScript { }
impl ScriptData for SigScript { }
impl ScriptData for WitnessScript { }
impl ScriptData for RedeemScript { }
impl ScriptData for TapScript { }
