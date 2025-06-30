use std::collections::HashSet;
use std::env;
use std::net::Ipv4Addr;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime};
use log::info;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Installation {
    pub id: String
}

struct LastCreation {
    millis: u128,
    created: HashSet<Ulid>
}

fn last() -> &'static Mutex<LastCreation> {
    static LOCK: OnceLock<Mutex<LastCreation>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(LastCreation{millis: 0, created: HashSet::new()}))
}

const NODE_BITS: u16 = 8;

fn node_id() -> &'static u16 {
    static LOCK: OnceLock<u16> = OnceLock::new();
    LOCK.get_or_init(|| {
        let ip: Ipv4Addr = env::var("POD_IP").unwrap().parse().unwrap();
        let node_id: u16 = ip.octets()[3] as u16;
        let max_node_id: u16 = (1 << NODE_BITS) - 1;
        if node_id > max_node_id {
            panic!("node_id is too large for the specified number of bits: {node_id}, {max_node_id}");
        } else {
            info!(target: "bosca", "node_id: {node_id}");
        }
        node_id
    })
}

impl Installation {

    pub fn new() -> Installation {
        let last = last();
        let mut last_creation = last.lock().unwrap();
        loop {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_millis();
            let timebits = (timestamp & ((1 << 48) - 1)) as u64;
            let mut source = thread_rng();
            let random_msb = source.gen::<u16>();
            let lsb = source.gen::<u64>();
            let node_id = *node_id();
            let cleared_msb = random_msb & !(((1 << NODE_BITS) - 1) << (16 - NODE_BITS));
            let node_msb = cleared_msb | (node_id << (16 - NODE_BITS));
            let msb = (timebits << 16) | u64::from(node_msb);
            let id = Ulid::from((msb, lsb));
            if last_creation.millis != timestamp {
                last_creation.millis = timestamp;
                last_creation.created.clear();
                last_creation.created.insert(id);
            } else if last_creation.created.contains(&id) {
                continue;
            } else {
                last_creation.created.insert(id);
            }
            return Installation {
                id: id.to_string()
            }
        }
    }
}