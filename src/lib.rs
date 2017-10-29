//! The `snowflake` crate is an implement of [twitter's snowflake algorithm](https://github.com/twitter/snowflake)
//! written in rust. Currently it generate uuid as follows:  
//! 
//! - 1 bit for unused sign bit
//! - 41 bits for milliseconds timestamp since `std::time::UNIX_EPOCH`
//! - 10 bits for generator id:
//!     - 5 bits for datacenter id
//!     - 5 bits for worker id in specific datacenter
//! - rest 12 bits for sequence id generated in the same timestamp at the generator
//!
//! In fact, the bits of the each three information can flow depending on the bussiness,
//! as long as they can form a 64 bit that can be store in an `i64`  
//! 
//! TODO:  
//!
//! - make the bits of information configurable
//! - make the codes more neat
//!
//! Author by [h_ang!(J27);](mailto:hunagjj.27@qq.com)

// NOTE: this crate can be benchmarked by uncommenting the test feature attribute
// and the benchmark codes.
#![feature(test)]

// extern crate test;

// TODO(huangjj.27@qq.com): make the EPOCH can be set by configurations
use std::time::{SystemTime, Duration, UNIX_EPOCH};

// TODO(huangjj.27@qq.com): make the bits configurable
pub const WORKER_ID_BITS: i64 = 5;
pub const DATACENTER_ID_BITS:i64 = 5;
pub const SEQUENCE_BITS: i64 = 12;

// use bit operations to get max number of the item
const MAX_WORKER_ID: i64 = -1 ^ (-1 << WORKER_ID_BITS);
const MAX_DATACENTER_ID:i64 = -1 ^ (-1 << DATACENTER_ID_BITS);

// shift bits
const WORKER_ID_SHIFT: i64 = SEQUENCE_BITS;
const DATACENTER_ID_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_LEFT_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;

// masks
const SEQUENC_MASK:i64 = -1 ^ (-1 << SEQUENCE_BITS);


/// Who generates id. It stores some information used to identify who it is, and
/// which id it should generate next.
#[derive(Debug)]
pub struct SnowFlakeWorker {
    worker_id: i64,
    datacenter_id: i64,

    // the id sequence of the worker in current millisecond timestamp
    sequence: i64,

    // the timestamp of last id generation
    last_timestamp: SystemTime,
}

impl SnowFlakeWorker {
    /// generates a new worker, with identifying information.
    pub fn new(worker_id: i64, datacenter_id: i64) -> Self {
        assert!(0 <= worker_id && worker_id <= MAX_WORKER_ID);
        assert!(0 <= datacenter_id && datacenter_id <= MAX_DATACENTER_ID);
        
        SnowFlakeWorker {
            worker_id,
            datacenter_id,
            sequence: 0,
            last_timestamp: UNIX_EPOCH,
        }
    }

    pub fn next_id(&mut self) -> i64 {
        let mut timestamp = SystemTime::now();
        assert!(timestamp >= self.last_timestamp);

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENC_MASK;

            // overflow and block until next millisecond
            if self.sequence == 0 {
                timestamp = self.block_for_new_millis();
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        // change the duration to an interger that we needed
        let duration = timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0,0));

        let duration = duration.as_secs() * 1000 + 
            (duration.subsec_nanos() / 1000) as u64;

        ((duration as i64) << TIMESTAMP_LEFT_SHIFT) |
        (self.datacenter_id << DATACENTER_ID_SHIFT) |
        (self.worker_id << WORKER_ID_SHIFT) |
        self.sequence
    }

    fn block_for_new_millis(&self) -> SystemTime {
        let mut now: SystemTime;
        loop {
            now = SystemTime::now();
            if now > self.last_timestamp {
                return now;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let mut worker = SnowFlakeWorker::new(0, 0);
        let new_id = worker.next_id();
        panic!("{:?}", new_id);
    }

    // #[bench]
    // fn bench_id_generation(bencher: &mut Bencher) {
    //     let mut worker = SnowFlakeWorker::new(0, 0);
    //     bencher.iter(|| worker.next_id());
    // }
}
