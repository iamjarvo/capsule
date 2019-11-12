/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

pub mod criterion;
mod packet;
pub mod proptest;
mod rvg;

pub mod byte_arrays {
    pub use crate::packets::icmp::v4::ICMPV4_PACKET;
    pub use crate::packets::icmp::v6::ICMPV6_PACKET;
    pub use crate::packets::ip::v6::{IPV6_PACKET, SRH_PACKET};
    pub use crate::packets::TCP_PACKET;
    pub use crate::packets::UDP_PACKET;
}

pub use self::packet::*;
pub use self::rvg::*;

use crate::dpdk::{self, Mempool, SocketId, MEMPOOL};
use std::ptr;
use std::sync::Once;

static TEST_INIT: Once = Once::new();

/// Run once initialization of EAL for `cargo test`.
pub fn cargo_test_init() {
    TEST_INIT.call_once(|| {
        dpdk::eal_init(vec!["nb2_test".to_owned()]).unwrap();
    });
}

/// A handle that keeps the mempool in scope for the duration of the test. It
/// will unset the thread-bound mempool on drop.
pub struct MempoolGuard {
    #[allow(dead_code)]
    inner: Mempool,
}

impl Drop for MempoolGuard {
    fn drop(&mut self) {
        MEMPOOL.with(|tls| tls.replace(ptr::null_mut()));
    }
}

/// Creates a new mempool for test that automatically cleans up after the
/// test completes.
pub fn new_mempool(capacity: usize, cache_size: usize) -> MempoolGuard {
    let mut mempool = Mempool::new(capacity, cache_size, SocketId::ANY).unwrap();
    MEMPOOL.with(|tls| tls.set(mempool.raw_mut()));
    MempoolGuard { inner: mempool }
}
