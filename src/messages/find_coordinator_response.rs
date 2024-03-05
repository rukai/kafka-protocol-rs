//! FindCoordinatorResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/FindCoordinatorResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct Coordinator {
    /// The coordinator key.
    ///
    /// Supported API versions: 4
    pub key: StrBytes,

    /// The node id.
    ///
    /// Supported API versions: 4
    pub node_id: super::BrokerId,

    /// The host name.
    ///
    /// Supported API versions: 4
    pub host: StrBytes,

    /// The port.
    ///
    /// Supported API versions: 4
    pub port: i32,

    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 4
    pub error_code: i16,

    /// The error message, or null if there was no error.
    ///
    /// Supported API versions: 4
    pub error_message: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for Coordinator {
    type Builder = CoordinatorBuilder;

    fn builder() -> Self::Builder {
        CoordinatorBuilder::default()
    }
}

impl Encodable for Coordinator {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, &self.key)?;
        } else {
            if !self.key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Int32.encode(buf, &self.node_id)?;
        } else {
            if self.node_id != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactString.encode(buf, &self.host)?;
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Int32.encode(buf, &self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactString.encode(buf, &self.error_message)?;
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.key)?;
        } else {
            if !self.key.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Int32.compute_size(&self.node_id)?;
        } else {
            if self.node_id != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.host)?;
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Int32.compute_size(&self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for Coordinator {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let key = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let node_id = if version >= 4 {
            types::Int32.decode(buf)?
        } else {
            (0).into()
        };
        let host = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let port = if version >= 4 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = if version >= 4 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let error_message = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            key,
            node_id,
            host,
            port,
            error_code,
            error_message,
            unknown_tagged_fields,
        })
    }
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            key: Default::default(),
            node_id: (0).into(),
            host: Default::default(),
            port: 0,
            error_code: 0,
            error_message: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for Coordinator {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 1-4
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The error message, or null if there was no error.
    ///
    /// Supported API versions: 1-3
    pub error_message: Option<StrBytes>,

    /// The node id.
    ///
    /// Supported API versions: 0-3
    pub node_id: super::BrokerId,

    /// The host name.
    ///
    /// Supported API versions: 0-3
    pub host: StrBytes,

    /// The port.
    ///
    /// Supported API versions: 0-3
    pub port: i32,

    /// Each coordinator result in the response
    ///
    /// Supported API versions: 4
    pub coordinators: Vec<Coordinator>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for FindCoordinatorResponse {
    type Builder = FindCoordinatorResponseBuilder;

    fn builder() -> Self::Builder {
        FindCoordinatorResponseBuilder::default()
    }
}

impl Encodable for FindCoordinatorResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version <= 3 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 1 && version <= 3 {
            if version >= 3 {
                types::CompactString.encode(buf, &self.error_message)?;
            } else {
                types::String.encode(buf, &self.error_message)?;
            }
        }
        if version <= 3 {
            types::Int32.encode(buf, &self.node_id)?;
        } else {
            if self.node_id != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                types::CompactString.encode(buf, &self.host)?;
            } else {
                types::String.encode(buf, &self.host)?;
            }
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            types::Int32.encode(buf, &self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.coordinators)?;
        } else {
            if !self.coordinators.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version <= 3 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 1 && version <= 3 {
            if version >= 3 {
                total_size += types::CompactString.compute_size(&self.error_message)?;
            } else {
                total_size += types::String.compute_size(&self.error_message)?;
            }
        }
        if version <= 3 {
            total_size += types::Int32.compute_size(&self.node_id)?;
        } else {
            if self.node_id != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            if version >= 3 {
                total_size += types::CompactString.compute_size(&self.host)?;
            } else {
                total_size += types::String.compute_size(&self.host)?;
            }
        } else {
            if !self.host.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 3 {
            total_size += types::Int32.compute_size(&self.port)?;
        } else {
            if self.port != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 4 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.coordinators)?;
        } else {
            if !self.coordinators.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for FindCoordinatorResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = if version <= 3 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let error_message = if version >= 1 && version <= 3 {
            if version >= 3 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let node_id = if version <= 3 {
            types::Int32.decode(buf)?
        } else {
            (0).into()
        };
        let host = if version <= 3 {
            if version >= 3 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let port = if version <= 3 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let coordinators = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            error_code,
            error_message,
            node_id,
            host,
            port,
            coordinators,
            unknown_tagged_fields,
        })
    }
}

impl Default for FindCoordinatorResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(Default::default()),
            node_id: (0).into(),
            host: Default::default(),
            port: 0,
            coordinators: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FindCoordinatorResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for FindCoordinatorResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            1
        } else {
            0
        }
    }
}
