use libp2p::kad::RoutingUpdate;
use libp2p::{Multiaddr, PeerId};
use libp2p::swarm::NetworkBehaviour;
use libp2p::kad::{
    Behaviour as KademliaBehavior,
    Event as KademliaEvent,
    store::MemoryStore as KademliaInMemory,
};

use libp2p::identify::{
    Behaviour as IdentifyBehavior, 
    Event as IdentifyEvent,
};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub(crate) struct Behavior {
    identify: IdentifyBehavior,
    kad: KademliaBehavior<KademliaInMemory>
}

impl Behavior {
    pub fn new(kad: KademliaBehavior<KademliaInMemory>, identify: IdentifyBehavior) -> Self {
        Self { kad, identify }
    }

    pub fn register_addr(&mut self, peer_id: &PeerId, addr: Multiaddr) -> RoutingUpdate {
        self.kad.add_address(peer_id, addr)
    }

    pub fn set_server_mode(&mut self) {
        self.kad.set_mode(Some(libp2p::kad::Mode::Server))
    }
}

#[derive(Debug)]
pub(crate) enum Event {
    Identify(IdentifyEvent),
    Kad(KademliaEvent)
}

impl From<IdentifyEvent> for Event {
    fn from(value: IdentifyEvent) -> Self {
        Self::Identify(value)
    }
}

impl From<KademliaEvent> for Event {
    fn from(value: KademliaEvent) -> Self {
        Self::Kad(value)
    }
}
