# PoC - Decentralized P2P Network Using rust-libp2p

This project is just my *midnight project* to explore, research and learn the latest technology relate with networking stacks especially for the P2P networks, using `libp2p`.

References:
- [libp2p](https://libp2p.io/)
- [rust-libp2p](https://docs.rs/libp2p/latest/libp2p/index.html)

## Overview

Through this project, I've been learned how to setup `rust-libp2p`, it's dependencies and how to use all of its components and building blocks, like `Network Behavior`, `Transport` and `Swarm`.

The use case that I've take is how to setup decentralized p2p networks by implementing two patterns:

- `Identify Pattern`
- `Kademlia DHT`

The journey is really tough in the beginning, but finally, I'm able to communicate three nodes on this p2p networks.

## Usages

Open three terminal windows

On the first terminal, run this command

```
cargo run
```

Example result:

```
   Compiling libp2p-kademlia v0.1.0 (/Users/hiraq/Projects/learn/rust/libp2p-kademlia)
    Finished dev [unoptimized + debuginfo] target(s) in 2.81s
     Running `target/debug/libp2p-kademlia`
[2023-11-11T15:23:47Z INFO  libp2p_kademlia] LocalPeerID: 12D3KooWGFMmCGJH8bSxrSX5ZZNPjcq64UcwpweWZVBx31uiP7C4
[2023-11-11T15:23:47Z INFO  libp2p_kademlia] Act as bootstrap node
[2023-11-11T15:23:47Z INFO  libp2p_kademlia] NewListenAddr: ListenerId(1) | "/ip4/127.0.0.1/tcp/62499"
[2023-11-11T15:23:47Z INFO  libp2p_kademlia] NewListenAddr: ListenerId(1) | "/ip4/192.168.100.139/tcp/62499"
```

In the second terminal, run this command

```
cargo run -- /ip4/127.0.0.1/tcp/62499 
```

Result:

```
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: sent confirmed protocol: /noise
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: confirming protocol: /yamux/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: sent confirmed protocol: /yamux/1.0.0
[2023-11-11T15:50:01Z DEBUG yamux::connection] new connection: d7b593df (Server)
[2023-11-11T15:50:01Z DEBUG yamux::connection] d7b593df: new outbound (Stream d7b593df/2) of (Connection d7b593df Server (streams 1))
[2023-11-11T15:50:01Z DEBUG multistream_select::dialer_select] Dialer: Proposed protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: confirming protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: sent confirmed protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] ConnectionEstablished: 12D3KooWNryLSn5EuR9KvCFbF65uSbL3YhFExEWDd3vY4TMnEbzu | 2 | Listener { local_addr: "/ip4/127.0.0.1/tcp/62500", send_back_addr: "/ip4/127.0.0.1/tcp/62646" } | 1 | None | 10.42375ms
[2023-11-11T15:50:01Z DEBUG multistream_select::dialer_select] Dialer: Received confirmation for protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] IdentifyReceived: Success register address
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] KadEvent:RoutingUpdated: 12D3KooWNryLSn5EuR9KvCFbF65uSbL3YhFExEWDd3vY4TMnEbzu | true | ["/ip4/127.0.0.1/tcp/62645"] | (Distance(57896044618658097711785492504343953926634992332820282019728792003956564819968), Distance(115792089237316195423570985008687907853269984665640564039457584007913129639935)) | None
```

In the third terminal, run this command

```
cargo run -- /ip4/127.0.0.1/tcp/62500 
```

Result

```
[2023-11-11T15:50:01Z DEBUG yamux::connection] new connection: 8f449576 (Client)
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] ConnectionEstablished: 12D3KooWKraqqEB5UdjUYVVLYZJqBYHTnr7vEHjxRTi3FK2t8E8S | 1 | Dialer { address: "/ip4/127.0.0.1/tcp/62500", role_override: Dialer } | 1 | Some([]) | 22.630084ms
[2023-11-11T15:50:01Z DEBUG yamux::connection] 8f449576: new outbound (Stream 8f449576/1) of (Connection 8f449576 Client (streams 0))
[2023-11-11T15:50:01Z DEBUG multistream_select::dialer_select] Dialer: Proposed protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::negotiated] Negotiated: Received confirmation for protocol: /yamux/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: confirming protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::listener_select] Listener: sent confirmed protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z DEBUG multistream_select::dialer_select] Dialer: Received confirmation for protocol: /ipfs/id/1.0.0
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] IdentifyReceived: Success register address
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] KadEvent:RoutingUpdated: 12D3KooWKraqqEB5UdjUYVVLYZJqBYHTnr7vEHjxRTi3FK2t8E8S | true | ["/ip4/192.168.100.139/tcp/62500"] | (Distance(57896044618658097711785492504343953926634992332820282019728792003956564819968), Distance(115792089237316195423570985008687907853269984665640564039457584007913129639935)) | None
[2023-11-11T15:50:01Z INFO  libp2p_kademlia] KadEvent:RoutingUpdated: 12D3KooWKraqqEB5UdjUYVVLYZJqBYHTnr7vEHjxRTi3FK2t8E8S | false | ["/ip4/192.168.100.139/tcp/62500", "/ip4/127.0.0.1/tcp/62500"] | (Distance(57896044618658097711785492504343953926634992332820282019728792003956564819968), Distance(115792089237316195423570985008687907853269984665640564039457584007913129639935)) | None
```