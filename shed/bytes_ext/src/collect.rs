/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License found in the LICENSE file in the root
 * directory of this source tree.
 */

use bytes::{BufMut, Bytes, BytesMut};

/// Wrapper for using Bytes in futures::TryStreamExt::try_collect which requires
/// a trait bound T: Default + Extend<Self\::Ok>. With this wrapper we get to
/// try_collect a stream of Bytes such as produced by Hyper clients.
///
/// More explicitly, if `resp` is a `hyper::Response<hyper::Body>` then we write:
/// ```ignore
///     resp.into_body().try_collect::<BytesCollect>().into()
/// ```
/// to get back Bytes.
#[derive(Default)]
pub struct BytesCollect {
    buffer: BytesMut,
}

impl BytesCollect {
    /// Create default instance of BytesCollect
    pub fn new() -> Self {
        Self::default()
    }
}

impl Extend<Bytes> for BytesCollect {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Bytes>,
    {
        for bytes in iter {
            self.buffer.put(bytes);
        }
    }
}

impl From<BytesCollect> for Bytes {
    fn from(collect: BytesCollect) -> Self {
        collect.buffer.freeze()
    }
}
