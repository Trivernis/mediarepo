use multihash::{Code, MultihashDigest};

use crate::error::RepoResult;

/// Creates a new content descriptor for the given file
pub fn create_content_descriptor(bytes: &[u8]) -> Vec<u8> {
    Code::Sha2_256.digest(bytes).to_bytes()
}

/// Encodes a content descriptor while respecting the version
pub fn encode_content_descriptor(descriptor: &[u8]) -> String {
    if is_v1_content_descriptor(descriptor) {
        encode_content_descriptor_v1(descriptor)
    } else {
        encode_content_descriptor_v2(descriptor)
    }
}

/// Encodes a v1 descriptor that is already stored encoded in the database (only interprets it as string)
pub fn encode_content_descriptor_v1(descriptor: &[u8]) -> String {
    String::from_utf8_lossy(descriptor).to_string()
}

/// Encodes the content descriptor as base32 lowercase
pub fn encode_content_descriptor_v2(descriptor: &[u8]) -> String {
    data_encoding::BASE32_DNSSEC.encode(descriptor)
}

/// Decodes a content descriptor
pub fn decode_content_descriptor<S: AsRef<str>>(descriptor: S) -> RepoResult<Vec<u8>> {
    // check for v1 descriptor with a fixed length of 53 starting with the prefix of the base and hash
    if is_v1_content_descriptor_string(descriptor.as_ref()) {
        decode_content_descriptor_v1(descriptor)
    } else {
        decode_content_descriptor_v2(descriptor)
    }
}

/// Decodes the first version of content descriptors (multibase)
pub fn decode_content_descriptor_v1<S: AsRef<str>>(descriptor: S) -> RepoResult<Vec<u8>> {
    Ok(descriptor.as_ref().as_bytes().to_vec())
}

/// Decodes the second version of content descriptors (faster fixed base32)
pub fn decode_content_descriptor_v2<S: AsRef<str>>(descriptor: S) -> RepoResult<Vec<u8>> {
    let data = data_encoding::BASE32_DNSSEC.decode(descriptor.as_ref().as_bytes())?;

    Ok(data)
}

/// Decodes the data stored in the v1 content descriptor into the v2 format
pub fn convert_v1_descriptor_to_v2(descriptor_v1: &[u8]) -> RepoResult<Vec<u8>> {
    let (_, data) = multibase::decode(encode_content_descriptor_v1(descriptor_v1))?;

    Ok(data)
}

/// Checks if a binary descriptor is v1
pub fn is_v1_content_descriptor(descriptor: &[u8]) -> bool {
    descriptor.len() == 56 && descriptor.starts_with(b"bciq")
}

/// Checks if a descriptor string is a v1 descriptor
pub fn is_v1_content_descriptor_string<S: AsRef<str>>(descriptor: S) -> bool {
    descriptor.as_ref().len() == 56 && descriptor.as_ref().starts_with("bciq")
}
