// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use std::{
	collections::{HashMap, HashSet},
	convert::TryFrom,
};

pub use sc_network::{PeerId, ReputationChange};

use polkadot_node_network_protocol::{ObservedRole, OurView, View, WrongVariant};
use polkadot_primitives::v2::{AuthorityDiscoveryId, SessionIndex, ValidatorIndex};

/// Information about a peer in the gossip topology for a session.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologyPeerInfo {
	/// The validator's known peer IDs.
	pub peer_ids: Vec<PeerId>,
	/// The index of the validator in the discovery keys of the corresponding
	/// `SessionInfo`. This can extend _beyond_ the set of active parachain validators.
	pub validator_index: ValidatorIndex,
}

/// A struct indicating new gossip topology.
#[derive(Debug, Clone, PartialEq)]
pub struct NewGossipTopology {
	/// The session index this topology corresponds to.
	pub session: SessionIndex,
	/// Neighbors in the 'X' dimension of the grid.
	pub our_neighbors_x: HashMap<AuthorityDiscoveryId, TopologyPeerInfo>,
	/// Neighbors in the 'Y' dimension of the grid.
	pub our_neighbors_y: HashMap<AuthorityDiscoveryId, TopologyPeerInfo>,
}

/// Events from network.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkBridgeEvent<M> {
	/// A peer has connected.
	PeerConnected(PeerId, ObservedRole, Option<HashSet<AuthorityDiscoveryId>>),

	/// A peer has disconnected.
	PeerDisconnected(PeerId),

	/// Our neighbors in the new gossip topology for the session.
	/// We're not necessarily connected to all of them.
	///
	/// This message is issued only on the validation peer set.
	///
	/// Note, that the distribution subsystems need to handle the last
	/// view update of the newly added gossip peers manually.
	NewGossipTopology(NewGossipTopology),

	/// Peer has sent a message.
	PeerMessage(PeerId, M),

	/// Peer's `View` has changed.
	PeerViewChange(PeerId, View),

	/// Our view has changed.
	OurViewChange(OurView),
}

impl<M> NetworkBridgeEvent<M> {
	/// Focus an overarching network-bridge event into some more specific variant.
	///
	/// This tries to transform M in `PeerMessage` to a message type specific to a subsystem.
	/// It is used to dispatch events coming from a peer set to the various subsystems that are
	/// handled within that peer set. More concretely a `ValidationProtocol` will be transformed
	/// for example into a `BitfieldDistributionMessage` in case of the `BitfieldDistribution`
	/// constructor.
	///
	/// Therefore a `NetworkBridgeEvent<ValidationProtocol>` will become for example a
	/// `NetworkBridgeEvent<BitfieldDistributionMessage>`, with the more specific message type
	/// `BitfieldDistributionMessage`.
	///
	/// This acts as a call to `clone`, except in the case where the event is a message event,
	/// in which case the clone can be expensive and it only clones if the message type can
	/// be focused.
	pub fn focus<'a, T>(&'a self) -> Result<NetworkBridgeEvent<T>, WrongVariant>
	where
		T: 'a + Clone,
		&'a T: TryFrom<&'a M, Error = WrongVariant>,
	{
		Ok(match *self {
			NetworkBridgeEvent::PeerMessage(ref peer, ref msg) =>
				NetworkBridgeEvent::PeerMessage(peer.clone(), <&'a T>::try_from(msg)?.clone()),
			NetworkBridgeEvent::PeerConnected(ref peer, ref role, ref authority_id) =>
				NetworkBridgeEvent::PeerConnected(peer.clone(), role.clone(), authority_id.clone()),
			NetworkBridgeEvent::PeerDisconnected(ref peer) =>
				NetworkBridgeEvent::PeerDisconnected(peer.clone()),
			NetworkBridgeEvent::NewGossipTopology(ref topology) =>
				NetworkBridgeEvent::NewGossipTopology(topology.clone()),
			NetworkBridgeEvent::PeerViewChange(ref peer, ref view) =>
				NetworkBridgeEvent::PeerViewChange(peer.clone(), view.clone()),
			NetworkBridgeEvent::OurViewChange(ref view) =>
				NetworkBridgeEvent::OurViewChange(view.clone()),
		})
	}
}
