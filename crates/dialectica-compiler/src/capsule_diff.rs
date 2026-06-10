use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use dialectica_capsule::PraxisCapsulePackage;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{digest_directory, read_jsonl_values, write_json, write_text, CompilerError};

pub const CAPSULE_DIFF_SCHEMA_VERSION: &str = "capsule_diff_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleDiffReceipt {
    pub diff_id: String,
    pub output_dir: PathBuf,
    pub diff_path: PathBuf,
    pub change_memo_path: PathBuf,
    pub added_claim_count: usize,
    pub retracted_claim_count: usize,
    pub superseded_claim_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleDiff {
    pub schema_version: String,
    pub diff_id: String,
    pub old_capsule: CapsuleDiffEndpoint,
    pub new_capsule: CapsuleDiffEndpoint,
    pub summary: CapsuleDiffSummary,
    pub claims: RecordFamilyDelta,
    pub sources: SourcePackDelta,
    pub review_transitions: Vec<FieldTransition>,
    pub trust_transitions: Vec<FieldTransition>,
    pub temporal_transitions: Vec<FieldTransition>,
    pub episode_boundary_changes: Vec<RecordChange>,
    pub reasoning_layer_delta: ReasoningLayerDelta,
    pub contradictions: ContradictionDelta,
    pub commitments: CommitmentDelta,
    pub citations: DiffCitations,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleDiffEndpoint {
    pub capsule_id: String,
    pub version: u64,
    pub capsule_type: String,
    pub title: String,
    pub bundle_digest: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleDiffSummary {
    pub added_claim_count: usize,
    pub retracted_claim_count: usize,
    pub superseded_claim_count: usize,
    pub source_added_count: usize,
    pub source_removed_count: usize,
    pub source_changed_count: usize,
    pub review_transition_count: usize,
    pub trust_transition_count: usize,
    pub temporal_transition_count: usize,
    pub episode_boundary_change_count: usize,
    pub reasoning_device_added_count: usize,
    pub reasoning_device_revised_count: usize,
    pub reasoning_device_retracted_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RecordFamilyDelta {
    pub added: Vec<RecordRef>,
    pub retracted: Vec<RecordRef>,
    pub superseded: Vec<RecordChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RecordRef {
    pub record_id: String,
    pub record_family: String,
    pub text: Option<String>,
    pub record_hash: String,
    pub source_hashes: Vec<String>,
    pub review_receipt_ids: Vec<String>,
    pub trust_layer: Option<String>,
    pub review_state: Option<String>,
    pub temporal_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordChange {
    pub record_id: String,
    pub record_family: String,
    pub old_record_hash: String,
    pub new_record_hash: String,
    pub changed_fields: Vec<String>,
    pub source_hashes: Vec<String>,
    pub review_receipt_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SourcePackDelta {
    pub added: Vec<SourceRef>,
    pub removed: Vec<SourceRef>,
    pub changed: Vec<SourceChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceRef {
    pub source_id: String,
    pub source_hash: String,
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceChange {
    pub source_id: String,
    pub old_source_hash: String,
    pub new_source_hash: String,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FieldTransition {
    pub record_id: String,
    pub record_family: String,
    pub field: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub source_hashes: Vec<String>,
    pub review_receipt_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReasoningLayerDelta {
    pub devices: RecordFamilyDelta,
    pub heuristics: RecordFamilyDelta,
    pub traps: RecordFamilyDelta,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ContradictionDelta {
    pub new_contradictions: Vec<String>,
    pub resolved_contradictions: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CommitmentDelta {
    pub lifecycle_changes: Vec<FieldTransition>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DiffCitations {
    pub new_source_hashes: Vec<String>,
    pub new_review_receipt_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct CapsuleSnapshot {
    package: PraxisCapsulePackage,
    bundle_digest: String,
    claims: BTreeMap<String, RecordSnapshot>,
    sources: BTreeMap<String, SourceSnapshot>,
    episodes: BTreeMap<String, RecordSnapshot>,
    reasoning_devices: BTreeMap<String, RecordSnapshot>,
}

#[derive(Debug, Clone)]
struct RecordSnapshot {
    id: String,
    family: String,
    value: Value,
    value_hash: String,
    text: Option<String>,
    source_hashes: Vec<String>,
    review_receipt_ids: Vec<String>,
    trust_layer: Option<String>,
    review_state: Option<String>,
    temporal_status: Option<String>,
}

#[derive(Debug, Clone)]
struct SourceSnapshot {
    id: String,
    hash: String,
    title: Option<String>,
    uri: Option<String>,
    value: Value,
}

impl CapsuleSnapshot {
    fn load(package_dir: &Path) -> Result<Self, CompilerError> {
        let package = PraxisCapsulePackage::load_from_dir(package_dir)
            .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
        let report = package.validate();
        if report.has_errors() {
            return Err(CompilerError::InvalidInput(format!(
                "cannot diff invalid package {}: {:?}",
                package_dir.display(),
                report.findings
            )));
        }

        let source_values = read_jsonl_values(&package_dir.join("evidence/sources.jsonl"))?;
        let sources = source_map(source_values)?;
        let source_hash_by_id = source_hash_lookup(&sources);
        let claim_values = read_jsonl_values(&package_dir.join("claims.jsonl"))?;
        let claims = record_map("claim", claim_values, &source_hash_by_id)?;
        let episode_values = read_episodes(&package_dir.join("episodes.json"))?;
        let episodes = record_map("episode", episode_values, &source_hash_by_id)?;
        let device_values: Vec<Value> = serde_json::from_str(&fs::read_to_string(
            package_dir.join("reasoning/devices.json"),
        )?)?;
        let reasoning_devices = record_map("reasoning_device", device_values, &source_hash_by_id)?;

        Ok(Self {
            package,
            bundle_digest: digest_directory(package_dir)?,
            claims,
            sources,
            episodes,
            reasoning_devices,
        })
    }

    fn endpoint(&self) -> CapsuleDiffEndpoint {
        CapsuleDiffEndpoint {
            capsule_id: self.package.manifest.capsule_id.clone(),
            version: self.package.manifest.version,
            capsule_type: self.package.manifest.capsule_type.clone(),
            title: self.package.manifest.title.clone(),
            bundle_digest: self.bundle_digest.clone(),
        }
    }
}

pub fn diff_capsules(old_dir: &Path, new_dir: &Path) -> Result<CapsuleDiff, CompilerError> {
    let old = CapsuleSnapshot::load(old_dir)?;
    let new = CapsuleSnapshot::load(new_dir)?;
    let claims = diff_records(&old.claims, &new.claims);
    let sources = diff_sources(&old.sources, &new.sources);
    let reasoning_devices = diff_records(&old.reasoning_devices, &new.reasoning_devices);
    let review_transitions = field_transitions(
        "review_state",
        ["claim", "reasoning_device", "episode"],
        &old,
        &new,
    );
    let trust_transitions = field_transitions("trust_layer", ["claim"], &old, &new);
    let temporal_transitions =
        field_transitions("temporal_status", ["claim", "episode"], &old, &new);
    let episode_boundary_changes = episode_boundary_changes(&old.episodes, &new.episodes);
    let citations = citations_for_new_delta(&claims, &reasoning_devices, &review_transitions);
    let summary = CapsuleDiffSummary {
        added_claim_count: claims.added.len(),
        retracted_claim_count: claims.retracted.len(),
        superseded_claim_count: claims.superseded.len(),
        source_added_count: sources.added.len(),
        source_removed_count: sources.removed.len(),
        source_changed_count: sources.changed.len(),
        review_transition_count: review_transitions.len(),
        trust_transition_count: trust_transitions.len(),
        temporal_transition_count: temporal_transitions.len(),
        episode_boundary_change_count: episode_boundary_changes.len(),
        reasoning_device_added_count: reasoning_devices.added.len(),
        reasoning_device_revised_count: reasoning_devices.superseded.len(),
        reasoning_device_retracted_count: reasoning_devices.retracted.len(),
    };
    let old_endpoint = old.endpoint();
    let new_endpoint = new.endpoint();

    Ok(CapsuleDiff {
        schema_version: CAPSULE_DIFF_SCHEMA_VERSION.to_owned(),
        diff_id: diff_id(&old_endpoint, &new_endpoint),
        old_capsule: old_endpoint,
        new_capsule: new_endpoint,
        summary,
        claims,
        sources,
        review_transitions,
        trust_transitions,
        temporal_transitions,
        episode_boundary_changes,
        reasoning_layer_delta: ReasoningLayerDelta {
            devices: reasoning_devices,
            heuristics: RecordFamilyDelta::default(),
            traps: RecordFamilyDelta::default(),
        },
        contradictions: ContradictionDelta::default(),
        commitments: CommitmentDelta::default(),
        citations,
    })
}

pub fn write_capsule_diff(
    old_dir: &Path,
    new_dir: &Path,
    output_dir: &Path,
) -> Result<CapsuleDiffReceipt, CompilerError> {
    let diff = diff_capsules(old_dir, new_dir)?;
    fs::create_dir_all(output_dir)?;
    let diff_path = output_dir.join("diff.json");
    let change_memo_path = output_dir.join("change-memo.md");
    write_json(&diff_path, &diff)?;
    write_text(&change_memo_path, &render_change_memo(&diff))?;

    Ok(CapsuleDiffReceipt {
        diff_id: diff.diff_id,
        output_dir: output_dir.to_path_buf(),
        diff_path,
        change_memo_path,
        added_claim_count: diff.summary.added_claim_count,
        retracted_claim_count: diff.summary.retracted_claim_count,
        superseded_claim_count: diff.summary.superseded_claim_count,
    })
}

pub fn render_change_memo(diff: &CapsuleDiff) -> String {
    let mut memo = String::new();
    memo.push_str("# Capsule Change Memo\n\n");
    memo.push_str(&format!("Diff: `{}`\n\n", diff.diff_id));
    memo.push_str(&format!(
        "- Old capsule: `{}` v{} ({})\n",
        diff.old_capsule.capsule_id, diff.old_capsule.version, diff.old_capsule.bundle_digest
    ));
    memo.push_str(&format!(
        "- New capsule: `{}` v{} ({})\n\n",
        diff.new_capsule.capsule_id, diff.new_capsule.version, diff.new_capsule.bundle_digest
    ));
    memo.push_str("## Summary\n\n");
    memo.push_str(&format!(
        "- Claims: {} added, {} retracted, {} superseded.\n",
        diff.summary.added_claim_count,
        diff.summary.retracted_claim_count,
        diff.summary.superseded_claim_count
    ));
    memo.push_str(&format!(
        "- Sources: {} added, {} removed, {} changed.\n",
        diff.summary.source_added_count,
        diff.summary.source_removed_count,
        diff.summary.source_changed_count
    ));
    memo.push_str(&format!(
        "- Review transitions: {}; trust transitions: {}; temporal transitions: {}.\n",
        diff.summary.review_transition_count,
        diff.summary.trust_transition_count,
        diff.summary.temporal_transition_count
    ));
    memo.push_str(&format!(
        "- Reasoning devices: {} added, {} revised, {} retracted.\n\n",
        diff.summary.reasoning_device_added_count,
        diff.summary.reasoning_device_revised_count,
        diff.summary.reasoning_device_retracted_count
    ));

    append_record_refs(&mut memo, "## Added Claims", &diff.claims.added);
    append_record_refs(&mut memo, "## Retracted Claims", &diff.claims.retracted);
    append_record_changes(&mut memo, "## Superseded Claims", &diff.claims.superseded);
    append_transitions(&mut memo, "## Review Transitions", &diff.review_transitions);
    append_transitions(&mut memo, "## Trust Transitions", &diff.trust_transitions);
    append_transitions(
        &mut memo,
        "## Temporal Transitions",
        &diff.temporal_transitions,
    );
    append_record_changes(
        &mut memo,
        "## Episode Boundary Changes",
        &diff.episode_boundary_changes,
    );
    append_record_refs(
        &mut memo,
        "## Added Reasoning Devices",
        &diff.reasoning_layer_delta.devices.added,
    );
    append_record_changes(
        &mut memo,
        "## Revised Reasoning Devices",
        &diff.reasoning_layer_delta.devices.superseded,
    );
    append_record_refs(
        &mut memo,
        "## Retracted Reasoning Devices",
        &diff.reasoning_layer_delta.devices.retracted,
    );

    memo.push_str("## Citation Receipts\n\n");
    if diff.citations.new_source_hashes.is_empty()
        && diff.citations.new_review_receipt_ids.is_empty()
    {
        memo.push_str("- No new source or review receipt citations were required.\n");
    } else {
        for source_hash in &diff.citations.new_source_hashes {
            memo.push_str(&format!("- Source hash: `{source_hash}`\n"));
        }
        for receipt_id in &diff.citations.new_review_receipt_ids {
            memo.push_str(&format!("- Review receipt: `{receipt_id}`\n"));
        }
    }
    memo
}

pub fn export_schema_dir(path: &Path) -> Result<(), CompilerError> {
    fs::create_dir_all(path)?;
    write_json(
        path.join("capsule_diff.schema.json").as_path(),
        &schema_for!(CapsuleDiff),
    )
}

fn source_map(values: Vec<Value>) -> Result<BTreeMap<String, SourceSnapshot>, CompilerError> {
    let mut map = BTreeMap::new();
    for value in values {
        let id = record_id(
            &value,
            &["source_id", "span_id", "document_id", "id"],
            "source",
        )?;
        let hash = first_string(
            &value,
            &["content_hash", "text_hash", "source_hash", "hash"],
        )
        .unwrap_or_else(|| digest_value(&value));
        let snapshot = SourceSnapshot {
            id: id.clone(),
            hash,
            title: first_string(&value, &["title"]),
            uri: first_string(&value, &["uri"]),
            value,
        };
        map.insert(id, snapshot);
    }
    Ok(map)
}

fn source_hash_lookup(sources: &BTreeMap<String, SourceSnapshot>) -> BTreeMap<String, String> {
    let mut lookup = BTreeMap::new();
    for source in sources.values() {
        lookup.insert(source.id.clone(), source.hash.clone());
        for key in ["source_id", "span_id", "document_id", "id"] {
            if let Some(id) = source.value.get(key).and_then(Value::as_str) {
                lookup.insert(id.to_owned(), source.hash.clone());
            }
        }
    }
    lookup
}

fn record_map(
    family: &str,
    values: Vec<Value>,
    source_hash_by_id: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, RecordSnapshot>, CompilerError> {
    let mut map = BTreeMap::new();
    for value in values {
        let id = match family {
            "claim" => record_id(&value, &["claim_id", "id", "record_id"], family)?,
            "episode" => record_id(&value, &["episode_id", "id", "record_id"], family)?,
            "reasoning_device" => record_id(&value, &["device_id", "id", "record_id"], family)?,
            _ => record_id(&value, &["id", "record_id"], family)?,
        };
        let source_hashes = source_refs(&value)
            .into_iter()
            .filter_map(|source_id| source_hash_by_id.get(&source_id).cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let review_receipt_ids = string_array_fields(&value, &["review_action_ids"])
            .into_iter()
            .chain(first_string(&value, &["review_decision_id", "review_id"]))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let snapshot = RecordSnapshot {
            id: id.clone(),
            family: family.to_owned(),
            value_hash: digest_value(&value),
            text: first_string(&value, &["claim_text", "text", "label", "purpose"]),
            source_hashes,
            review_receipt_ids,
            trust_layer: first_string(&value, &["trust_layer", "trust_status"]),
            review_state: first_string(&value, &["review_state", "review_status"]),
            temporal_status: first_string(&value, &["temporal_status", "status"]),
            value,
        };
        map.insert(id, snapshot);
    }
    Ok(map)
}

fn read_episodes(path: &Path) -> Result<Vec<Value>, CompilerError> {
    let value: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    if let Some(episodes) = value.get("episodes").and_then(Value::as_array) {
        Ok(episodes.clone())
    } else if let Some(episodes) = value.as_array() {
        Ok(episodes.clone())
    } else {
        Ok(Vec::new())
    }
}

fn diff_records(
    old: &BTreeMap<String, RecordSnapshot>,
    new: &BTreeMap<String, RecordSnapshot>,
) -> RecordFamilyDelta {
    let added = new
        .iter()
        .filter(|(id, _)| !old.contains_key(*id))
        .map(|(_, record)| record_ref(record))
        .collect();
    let retracted = old
        .iter()
        .filter(|(id, _)| !new.contains_key(*id))
        .map(|(_, record)| record_ref(record))
        .collect();
    let superseded = old
        .iter()
        .filter_map(|(id, old_record)| {
            let new_record = new.get(id)?;
            if old_record.value_hash == new_record.value_hash {
                return None;
            }
            Some(record_change(old_record, new_record))
        })
        .collect();
    RecordFamilyDelta {
        added,
        retracted,
        superseded,
    }
}

fn diff_sources(
    old: &BTreeMap<String, SourceSnapshot>,
    new: &BTreeMap<String, SourceSnapshot>,
) -> SourcePackDelta {
    let added = new
        .iter()
        .filter(|(id, _)| !old.contains_key(*id))
        .map(|(_, source)| source_ref(source))
        .collect();
    let removed = old
        .iter()
        .filter(|(id, _)| !new.contains_key(*id))
        .map(|(_, source)| source_ref(source))
        .collect();
    let changed = old
        .iter()
        .filter_map(|(id, old_source)| {
            let new_source = new.get(id)?;
            if old_source.hash == new_source.hash && old_source.value == new_source.value {
                return None;
            }
            Some(SourceChange {
                source_id: id.clone(),
                old_source_hash: old_source.hash.clone(),
                new_source_hash: new_source.hash.clone(),
                changed_fields: changed_fields(&old_source.value, &new_source.value),
            })
        })
        .collect();
    SourcePackDelta {
        added,
        removed,
        changed,
    }
}

fn field_transitions<const N: usize>(
    field: &str,
    families: [&str; N],
    old: &CapsuleSnapshot,
    new: &CapsuleSnapshot,
) -> Vec<FieldTransition> {
    let mut transitions = Vec::new();
    for family in families {
        let (old_records, new_records) = match family {
            "claim" => (&old.claims, &new.claims),
            "episode" => (&old.episodes, &new.episodes),
            "reasoning_device" => (&old.reasoning_devices, &new.reasoning_devices),
            _ => continue,
        };
        for (id, old_record) in old_records {
            let Some(new_record) = new_records.get(id) else {
                continue;
            };
            let (from, to) = match field {
                "review_state" => (&old_record.review_state, &new_record.review_state),
                "trust_layer" => (&old_record.trust_layer, &new_record.trust_layer),
                "temporal_status" => (&old_record.temporal_status, &new_record.temporal_status),
                _ => continue,
            };
            if from == to || (from.is_none() && to.is_none()) {
                continue;
            }
            transitions.push(FieldTransition {
                record_id: id.clone(),
                record_family: family.to_owned(),
                field: field.to_owned(),
                from: from.clone(),
                to: to.clone(),
                source_hashes: new_record.source_hashes.clone(),
                review_receipt_ids: new_record.review_receipt_ids.clone(),
            });
        }
    }
    transitions.sort_by(|left, right| {
        left.record_family
            .cmp(&right.record_family)
            .then(left.record_id.cmp(&right.record_id))
            .then(left.field.cmp(&right.field))
    });
    transitions
}

fn episode_boundary_changes(
    old: &BTreeMap<String, RecordSnapshot>,
    new: &BTreeMap<String, RecordSnapshot>,
) -> Vec<RecordChange> {
    let boundary_fields = ["valid_from", "valid_to", "status", "temporal_status"];
    old.iter()
        .filter_map(|(id, old_record)| {
            let new_record = new.get(id)?;
            let changed = changed_fields(&old_record.value, &new_record.value)
                .into_iter()
                .filter(|field| boundary_fields.contains(&field.as_str()))
                .collect::<Vec<_>>();
            if changed.is_empty() {
                return None;
            }
            Some(RecordChange {
                record_id: id.clone(),
                record_family: "episode".to_owned(),
                old_record_hash: old_record.value_hash.clone(),
                new_record_hash: new_record.value_hash.clone(),
                changed_fields: changed,
                source_hashes: new_record.source_hashes.clone(),
                review_receipt_ids: new_record.review_receipt_ids.clone(),
            })
        })
        .collect()
}

fn citations_for_new_delta(
    claims: &RecordFamilyDelta,
    reasoning_devices: &RecordFamilyDelta,
    review_transitions: &[FieldTransition],
) -> DiffCitations {
    let mut source_hashes = BTreeSet::new();
    let mut review_receipt_ids = BTreeSet::new();

    for record in claims.added.iter().chain(reasoning_devices.added.iter()) {
        source_hashes.extend(record.source_hashes.iter().cloned());
        review_receipt_ids.extend(record.review_receipt_ids.iter().cloned());
    }
    for change in claims
        .superseded
        .iter()
        .chain(reasoning_devices.superseded.iter())
    {
        source_hashes.extend(change.source_hashes.iter().cloned());
        review_receipt_ids.extend(change.review_receipt_ids.iter().cloned());
    }
    for transition in review_transitions {
        source_hashes.extend(transition.source_hashes.iter().cloned());
        review_receipt_ids.extend(transition.review_receipt_ids.iter().cloned());
    }
    DiffCitations {
        new_source_hashes: source_hashes.into_iter().collect(),
        new_review_receipt_ids: review_receipt_ids.into_iter().collect(),
    }
}

fn record_ref(record: &RecordSnapshot) -> RecordRef {
    RecordRef {
        record_id: record.id.clone(),
        record_family: record.family.clone(),
        text: record.text.clone(),
        record_hash: record.value_hash.clone(),
        source_hashes: record.source_hashes.clone(),
        review_receipt_ids: record.review_receipt_ids.clone(),
        trust_layer: record.trust_layer.clone(),
        review_state: record.review_state.clone(),
        temporal_status: record.temporal_status.clone(),
    }
}

fn record_change(old: &RecordSnapshot, new: &RecordSnapshot) -> RecordChange {
    RecordChange {
        record_id: new.id.clone(),
        record_family: new.family.clone(),
        old_record_hash: old.value_hash.clone(),
        new_record_hash: new.value_hash.clone(),
        changed_fields: changed_fields(&old.value, &new.value),
        source_hashes: new.source_hashes.clone(),
        review_receipt_ids: new.review_receipt_ids.clone(),
    }
}

fn source_ref(source: &SourceSnapshot) -> SourceRef {
    SourceRef {
        source_id: source.id.clone(),
        source_hash: source.hash.clone(),
        title: source.title.clone(),
        uri: source.uri.clone(),
    }
}

fn changed_fields(old: &Value, new: &Value) -> Vec<String> {
    let Some(old_object) = old.as_object() else {
        return if old == new {
            Vec::new()
        } else {
            vec!["$".to_owned()]
        };
    };
    let Some(new_object) = new.as_object() else {
        return if old == new {
            Vec::new()
        } else {
            vec!["$".to_owned()]
        };
    };
    old_object
        .keys()
        .chain(new_object.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|key| old_object.get(*key) != new_object.get(*key))
        .cloned()
        .collect()
}

fn record_id(value: &Value, keys: &[&str], family: &str) -> Result<String, CompilerError> {
    for key in keys {
        if let Some(id) = value.get(key).and_then(Value::as_str) {
            if !id.trim().is_empty() {
                return Ok(id.to_owned());
            }
        }
    }
    Err(CompilerError::InvalidInput(format!(
        "{family} record missing stable id"
    )))
}

fn first_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(key).and_then(Value::as_str).map(str::to_owned))
}

fn string_array_fields(value: &Value, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter_map(|key| value.get(key).and_then(Value::as_array))
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn source_refs(value: &Value) -> Vec<String> {
    let mut refs =
        string_array_fields(value, &["source_span_ids", "source_ids", "source_hash_ids"]);
    if let Some(source_id) = value
        .get("source_span")
        .and_then(|span| span.get("source_id"))
        .and_then(Value::as_str)
    {
        refs.push(source_id.to_owned());
    }
    if let Some(spans) = value.get("source_spans").and_then(Value::as_array) {
        refs.extend(spans.iter().filter_map(|span| {
            span.get("source_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        }));
    }
    refs.sort();
    refs.dedup();
    refs
}

fn digest_value(value: &Value) -> String {
    format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(value).expect("JSON value should serialize"))
    )
}

fn diff_id(old: &CapsuleDiffEndpoint, new: &CapsuleDiffEndpoint) -> String {
    format!(
        "diff_{}_{}_to_{}_{}",
        stable_id_component(&old.capsule_id),
        short_digest(&old.bundle_digest),
        stable_id_component(&new.capsule_id),
        short_digest(&new.bundle_digest)
    )
}

fn stable_id_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn short_digest(digest: &str) -> String {
    digest
        .trim_start_matches("sha256:")
        .chars()
        .take(12)
        .collect()
}

fn append_record_refs(memo: &mut String, heading: &str, records: &[RecordRef]) {
    memo.push_str(heading);
    memo.push_str("\n\n");
    if records.is_empty() {
        memo.push_str("- None.\n\n");
        return;
    }
    for record in records {
        let text = record.text.as_deref().unwrap_or("no text field");
        memo.push_str(&format!(
            "- `{}` ({}) {}",
            record.record_id, record.record_family, text
        ));
        append_inline_citations(memo, &record.source_hashes, &record.review_receipt_ids);
        memo.push('\n');
    }
    memo.push('\n');
}

fn append_record_changes(memo: &mut String, heading: &str, records: &[RecordChange]) {
    memo.push_str(heading);
    memo.push_str("\n\n");
    if records.is_empty() {
        memo.push_str("- None.\n\n");
        return;
    }
    for record in records {
        memo.push_str(&format!(
            "- `{}` ({}) changed fields: {}",
            record.record_id,
            record.record_family,
            record.changed_fields.join(", ")
        ));
        append_inline_citations(memo, &record.source_hashes, &record.review_receipt_ids);
        memo.push('\n');
    }
    memo.push('\n');
}

fn append_transitions(memo: &mut String, heading: &str, transitions: &[FieldTransition]) {
    memo.push_str(heading);
    memo.push_str("\n\n");
    if transitions.is_empty() {
        memo.push_str("- None.\n\n");
        return;
    }
    for transition in transitions {
        memo.push_str(&format!(
            "- `{}` ({}) `{}`: `{}` -> `{}`",
            transition.record_id,
            transition.record_family,
            transition.field,
            transition.from.as_deref().unwrap_or("unset"),
            transition.to.as_deref().unwrap_or("unset")
        ));
        append_inline_citations(
            memo,
            &transition.source_hashes,
            &transition.review_receipt_ids,
        );
        memo.push('\n');
    }
    memo.push('\n');
}

fn append_inline_citations(
    memo: &mut String,
    source_hashes: &[String],
    review_receipts: &[String],
) {
    if source_hashes.is_empty() && review_receipts.is_empty() {
        return;
    }
    let mut citations = Vec::new();
    citations.extend(source_hashes.iter().map(|hash| format!("source `{hash}`")));
    citations.extend(
        review_receipts
            .iter()
            .map(|receipt| format!("review `{receipt}`")),
    );
    memo.push_str(&format!(" [{}]", citations.join("; ")));
}
