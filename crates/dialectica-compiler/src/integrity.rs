use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use dialectica_capsule::PraxisCapsulePackage;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{list_package_files, write_json, CompilerError};

pub const INTEGRITY_ENVELOPE_PATH: &str = "integrity/envelope.json";
pub const INTEGRITY_SCHEMA_VERSION: &str = "integrity_envelope_v1";

const AUTHOR_KEY_LABEL: &str = "DIALECTICA local fixture integrity author key v1";
const PUBLISHER_KEY_LABEL: &str = "DIALECTICA local fixture integrity publisher key v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegrityEnvelope {
    pub schema_version: String,
    pub signature_payload: IntegritySignaturePayload,
    pub signature_payload_sha256: String,
    pub signatures: Vec<IntegritySignature>,
    pub dsse_reserved: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegritySignaturePayload {
    pub schema_version: String,
    pub capsule_id: String,
    pub spec_version: String,
    pub canonical_scope: IntegrityScope,
    pub leaves: Vec<IntegrityLeaf>,
    pub merkle_root: String,
    pub author: SignatureIdentity,
    pub publisher: SignatureIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegrityScope {
    pub hash_algorithm: String,
    pub merkle_algorithm: String,
    pub included_path_count: usize,
    pub excluded_paths: Vec<String>,
    pub excluded_path_prefixes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegrityLeaf {
    pub path: String,
    pub sha256: String,
    pub leaf_hash: String,
    pub byte_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SignatureIdentity {
    pub identity_id: String,
    pub display_name: String,
    pub role: String,
    pub public_key_ed25519_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegritySignature {
    pub signature_id: String,
    pub role: String,
    pub identity: SignatureIdentity,
    pub payload_sha256: String,
    pub signature_ed25519_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntegrityVerificationReport {
    pub capsule_id: String,
    pub verified: bool,
    pub checked_file_count: usize,
    pub merkle_root: String,
    pub envelope_merkle_root: String,
    pub signature_count: usize,
    pub findings: Vec<String>,
}

pub fn write_integrity_envelope(package_dir: &Path) -> Result<IntegrityEnvelope, CompilerError> {
    let payload = build_signature_payload(package_dir)?;
    let payload_bytes = serde_json::to_vec(&payload)?;
    let payload_sha256 = sha256_digest(&payload_bytes);
    let signatures = sign_payload(&payload, &payload_bytes, &payload_sha256);
    let envelope = IntegrityEnvelope {
        schema_version: INTEGRITY_SCHEMA_VERSION.to_owned(),
        signature_payload: payload,
        signature_payload_sha256: payload_sha256,
        signatures,
        dsse_reserved: json!({
            "payload_type": "application/vnd.tacitus.dialectica.integrity+json",
            "bundle": null
        }),
    };

    write_json(&package_dir.join(INTEGRITY_ENVELOPE_PATH), &envelope)?;
    Ok(envelope)
}

pub fn verify_integrity_envelope(
    package_dir: &Path,
) -> Result<IntegrityVerificationReport, CompilerError> {
    let envelope_path = package_dir.join(INTEGRITY_ENVELOPE_PATH);
    let envelope: IntegrityEnvelope = serde_json::from_str(&fs::read_to_string(&envelope_path)?)?;
    let current_payload = build_signature_payload(package_dir)?;
    let mut findings = Vec::new();

    compare_payloads(&envelope.signature_payload, &current_payload, &mut findings);

    let envelope_payload_bytes = serde_json::to_vec(&envelope.signature_payload)?;
    let envelope_payload_sha256 = sha256_digest(&envelope_payload_bytes);
    if envelope_payload_sha256 != envelope.signature_payload_sha256 {
        findings.push(format!(
            "signature payload hash mismatch: envelope={} computed={}",
            envelope.signature_payload_sha256, envelope_payload_sha256
        ));
    }

    verify_signatures(
        &envelope,
        &envelope_payload_bytes,
        &envelope_payload_sha256,
        &mut findings,
    );

    Ok(IntegrityVerificationReport {
        capsule_id: envelope.signature_payload.capsule_id,
        verified: findings.is_empty(),
        checked_file_count: current_payload.leaves.len(),
        merkle_root: current_payload.merkle_root,
        envelope_merkle_root: envelope.signature_payload.merkle_root,
        signature_count: envelope.signatures.len(),
        findings,
    })
}

pub fn export_integrity_schema_dir(path: &Path) -> Result<(), CompilerError> {
    fs::create_dir_all(path)?;
    write_json(
        path.join("integrity_envelope.schema.json").as_path(),
        &schema_for!(IntegrityEnvelope),
    )
}

fn build_signature_payload(package_dir: &Path) -> Result<IntegritySignaturePayload, CompilerError> {
    let package = PraxisCapsulePackage::load_from_dir(package_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let (author, _) = fixture_identity(
        "author",
        "fixture-author:local",
        "DIALECTICA local fixture author",
        AUTHOR_KEY_LABEL,
    );
    let (publisher, _) = fixture_identity(
        "publisher",
        "fixture-publisher:local",
        "DIALECTICA local fixture publisher",
        PUBLISHER_KEY_LABEL,
    );
    let mut leaves = integrity_leaves(package_dir)?;
    leaves.sort_by(|left, right| left.path.cmp(&right.path));
    let scope = IntegrityScope {
        hash_algorithm: "sha256".to_owned(),
        merkle_algorithm: "sha256-path-bound-binary-tree-v1".to_owned(),
        included_path_count: leaves.len(),
        excluded_paths: vec![INTEGRITY_ENVELOPE_PATH.to_owned()],
        excluded_path_prefixes: vec!["graph/ladybug/".to_owned()],
    };
    let merkle_root = merkle_root(&leaves)?;

    Ok(IntegritySignaturePayload {
        schema_version: INTEGRITY_SCHEMA_VERSION.to_owned(),
        capsule_id: package.manifest.capsule_id,
        spec_version: package.manifest.spec_version,
        canonical_scope: scope,
        leaves,
        merkle_root,
        author,
        publisher,
    })
}

fn integrity_leaves(package_dir: &Path) -> Result<Vec<IntegrityLeaf>, CompilerError> {
    list_package_files(package_dir)?
        .into_iter()
        .filter(|path| !is_excluded_from_integrity(path))
        .map(|path| {
            let bytes = fs::read(package_dir.join(PathBuf::from(&path)))?;
            let mut leaf_hasher = Sha256::new();
            leaf_hasher.update(path.as_bytes());
            leaf_hasher.update([0]);
            leaf_hasher.update(&bytes);
            Ok(IntegrityLeaf {
                path,
                sha256: sha256_digest(&bytes),
                leaf_hash: format!("sha256:{}", hex_encode(&leaf_hasher.finalize())),
                byte_count: bytes.len() as u64,
            })
        })
        .collect()
}

fn is_excluded_from_integrity(path: &str) -> bool {
    path == INTEGRITY_ENVELOPE_PATH || path.starts_with("graph/ladybug/")
}

fn merkle_root(leaves: &[IntegrityLeaf]) -> Result<String, CompilerError> {
    let mut level = leaves
        .iter()
        .map(|leaf| decode_sha256_digest(&leaf.leaf_hash))
        .collect::<Result<Vec<_>, _>>()?;
    if level.is_empty() {
        return Ok(sha256_digest(&[]));
    }

    while level.len() > 1 {
        let mut next = Vec::new();
        for chunk in level.chunks(2) {
            let left = chunk[0];
            let right = chunk.get(1).copied().unwrap_or(left);
            let mut hasher = Sha256::new();
            hasher.update(left);
            hasher.update(right);
            next.push(hasher.finalize().into());
        }
        level = next;
    }

    Ok(format!("sha256:{}", hex_encode(&level[0])))
}

fn sign_payload(
    payload: &IntegritySignaturePayload,
    payload_bytes: &[u8],
    payload_sha256: &str,
) -> Vec<IntegritySignature> {
    [
        (
            "author",
            payload.author.clone(),
            signing_key(AUTHOR_KEY_LABEL),
        ),
        (
            "publisher",
            payload.publisher.clone(),
            signing_key(PUBLISHER_KEY_LABEL),
        ),
    ]
    .into_iter()
    .map(|(role, identity, signing_key)| {
        let signature = signing_key.sign(payload_bytes);
        IntegritySignature {
            signature_id: format!(
                "sig_{}_{}",
                role,
                payload_sha256
                    .trim_start_matches("sha256:")
                    .chars()
                    .take(12)
                    .collect::<String>()
            ),
            role: role.to_owned(),
            identity,
            payload_sha256: payload_sha256.to_owned(),
            signature_ed25519_hex: hex_encode(&signature.to_bytes()),
        }
    })
    .collect()
}

fn verify_signatures(
    envelope: &IntegrityEnvelope,
    payload_bytes: &[u8],
    payload_sha256: &str,
    findings: &mut Vec<String>,
) {
    if envelope.signatures.is_empty() {
        findings.push("integrity envelope has no signatures".to_owned());
        return;
    }

    let expected_identities = [
        (&envelope.signature_payload.author, "author"),
        (&envelope.signature_payload.publisher, "publisher"),
    ]
    .into_iter()
    .map(|(identity, role)| (role.to_owned(), identity.clone()))
    .collect::<BTreeMap<_, _>>();

    for signature in &envelope.signatures {
        if signature.payload_sha256 != payload_sha256 {
            findings.push(format!(
                "signature {} payload hash mismatch: signature={} computed={}",
                signature.signature_id, signature.payload_sha256, payload_sha256
            ));
            continue;
        }

        let Some(expected_identity) = expected_identities.get(&signature.role) else {
            findings.push(format!(
                "signature {} has unsupported role {}",
                signature.signature_id, signature.role
            ));
            continue;
        };
        if &signature.identity != expected_identity {
            findings.push(format!(
                "signature {} identity does not match payload {} identity",
                signature.signature_id, signature.role
            ));
            continue;
        }

        let verifying_key = match decode_fixed_hex::<32>(&signature.identity.public_key_ed25519_hex)
            .and_then(|bytes| {
                VerifyingKey::from_bytes(&bytes)
                    .map_err(|error| CompilerError::InvalidInput(error.to_string()))
            }) {
            Ok(value) => value,
            Err(error) => {
                findings.push(format!(
                    "signature {} public key is invalid: {}",
                    signature.signature_id, error
                ));
                continue;
            }
        };
        let signature_bytes = match decode_fixed_hex::<64>(&signature.signature_ed25519_hex) {
            Ok(value) => value,
            Err(error) => {
                findings.push(format!(
                    "signature {} bytes are invalid: {}",
                    signature.signature_id, error
                ));
                continue;
            }
        };
        let signature_value = Signature::from_bytes(&signature_bytes);
        if let Err(error) = verifying_key.verify(payload_bytes, &signature_value) {
            findings.push(format!(
                "signature {} verification failed: {}",
                signature.signature_id, error
            ));
        }
    }

    for role in expected_identities.keys() {
        if !envelope
            .signatures
            .iter()
            .any(|signature| signature.role == *role)
        {
            findings.push(format!("missing {} signature", role));
        }
    }
}

fn compare_payloads(
    envelope_payload: &IntegritySignaturePayload,
    current_payload: &IntegritySignaturePayload,
    findings: &mut Vec<String>,
) {
    if envelope_payload.schema_version != current_payload.schema_version {
        findings.push(format!(
            "schema version mismatch: envelope={} current={}",
            envelope_payload.schema_version, current_payload.schema_version
        ));
    }
    if envelope_payload.capsule_id != current_payload.capsule_id {
        findings.push(format!(
            "capsule id mismatch: envelope={} current={}",
            envelope_payload.capsule_id, current_payload.capsule_id
        ));
    }
    if envelope_payload.spec_version != current_payload.spec_version {
        findings.push(format!(
            "spec version mismatch: envelope={} current={}",
            envelope_payload.spec_version, current_payload.spec_version
        ));
    }
    if envelope_payload.canonical_scope != current_payload.canonical_scope {
        findings.push("canonical integrity scope mismatch".to_owned());
    }
    if envelope_payload.merkle_root != current_payload.merkle_root {
        findings.push(format!(
            "merkle root mismatch: envelope={} current={}",
            envelope_payload.merkle_root, current_payload.merkle_root
        ));
    }
    if envelope_payload.author != current_payload.author {
        findings.push("author identity mismatch".to_owned());
    }
    if envelope_payload.publisher != current_payload.publisher {
        findings.push("publisher identity mismatch".to_owned());
    }
    compare_leaves(&envelope_payload.leaves, &current_payload.leaves, findings);
}

fn compare_leaves(
    envelope_leaves: &[IntegrityLeaf],
    current_leaves: &[IntegrityLeaf],
    findings: &mut Vec<String>,
) {
    let envelope_by_path = envelope_leaves
        .iter()
        .map(|leaf| (leaf.path.as_str(), leaf))
        .collect::<BTreeMap<_, _>>();
    let current_by_path = current_leaves
        .iter()
        .map(|leaf| (leaf.path.as_str(), leaf))
        .collect::<BTreeMap<_, _>>();

    for path in envelope_by_path.keys() {
        if !current_by_path.contains_key(path) {
            findings.push(format!("missing signed file: {path}"));
        }
    }
    for path in current_by_path.keys() {
        if !envelope_by_path.contains_key(path) {
            findings.push(format!("unsigned added file in canonical scope: {path}"));
        }
    }
    for (path, envelope_leaf) in &envelope_by_path {
        let Some(current_leaf) = current_by_path.get(path) else {
            continue;
        };
        if envelope_leaf.sha256 != current_leaf.sha256
            || envelope_leaf.leaf_hash != current_leaf.leaf_hash
            || envelope_leaf.byte_count != current_leaf.byte_count
        {
            findings.push(format!(
                "signed file changed: {} envelope_hash={} current_hash={}",
                path, envelope_leaf.sha256, current_leaf.sha256
            ));
        }
    }
}

fn fixture_identity(
    role: &str,
    identity_id: &str,
    display_name: &str,
    key_label: &str,
) -> (SignatureIdentity, SigningKey) {
    let signing_key = signing_key(key_label);
    let identity = SignatureIdentity {
        identity_id: identity_id.to_owned(),
        display_name: display_name.to_owned(),
        role: role.to_owned(),
        public_key_ed25519_hex: hex_encode(signing_key.verifying_key().as_bytes()),
    };
    (identity, signing_key)
}

fn signing_key(label: &str) -> SigningKey {
    let digest = Sha256::digest(label.as_bytes());
    let mut seed = [0_u8; 32];
    seed.copy_from_slice(&digest[..32]);
    SigningKey::from_bytes(&seed)
}

fn sha256_digest(bytes: &[u8]) -> String {
    format!("sha256:{}", hex_encode(&Sha256::digest(bytes)))
}

fn decode_sha256_digest(value: &str) -> Result<[u8; 32], CompilerError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(CompilerError::InvalidInput(format!(
            "digest {value} is missing sha256 prefix"
        )));
    };
    decode_fixed_hex::<32>(hex)
}

fn decode_fixed_hex<const N: usize>(hex: &str) -> Result<[u8; N], CompilerError> {
    let bytes = decode_hex(hex)?;
    if bytes.len() != N {
        return Err(CompilerError::InvalidInput(format!(
            "expected {} bytes of hex, got {}",
            N,
            bytes.len()
        )));
    }
    let mut output = [0_u8; N];
    output.copy_from_slice(&bytes);
    Ok(output)
}

fn decode_hex(hex: &str) -> Result<Vec<u8>, CompilerError> {
    if hex.len() % 2 != 0 {
        return Err(CompilerError::InvalidInput(
            "hex string has odd length".to_owned(),
        ));
    }
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let high = hex_nibble(chunk[0])?;
            let low = hex_nibble(chunk[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn hex_nibble(byte: u8) -> Result<u8, CompilerError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CompilerError::InvalidInput(format!(
            "invalid hex digit {}",
            byte as char
        ))),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
