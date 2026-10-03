//! Draft-only adapter. The installed PRAXIS shared runtime validates the final native artifact.
use crate::CompilerError;
use dialectica_extractor::{validate_source_pack, SourcePack};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CognitiveDraftRequest {
    pub contract_version: String,
    pub capsule_id: String,
    pub version: String,
    pub profile: String,
    #[serde(default)]
    pub nodes: Vec<Value>,
    #[serde(default)]
    pub statements: Vec<Value>,
    #[serde(default)]
    pub knowledge: Vec<Value>,
    #[serde(default)]
    pub dependencies: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NativeCognitiveReceipt {
    pub archive_path: PathBuf,
    pub archive_digest: String,
    pub schema_version: String,
    pub status: String,
    pub validated_by: String,
    pub canonical_writes: bool,
}

fn invalid(message: &str) -> CompilerError {
    CompilerError::InvalidInput(message.to_owned())
}
fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn is_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Compile explicitly supplied semantic proposals against the engine's original source pack.
pub fn compile_cognitive_draft(
    request: &CognitiveDraftRequest,
    source: &SourcePack,
) -> Result<Value, CompilerError> {
    if request.contract_version != "dialectica-cognitive-draft/1" {
        return Err(invalid("Unsupported cognitive draft contract."));
    }
    if request.nodes.len() > 5000
        || request.statements.len() > 10000
        || request.knowledge.len() > 2000
        || request.dependencies.len() > 100
        || source.spans.len() > 10000
        || source.documents.len() > 5000
    {
        return Err(invalid("Cognitive draft record budget exceeded."));
    }
    let validation = validate_source_pack(source);
    if validation.has_errors() {
        return Err(CompilerError::Validation(validation));
    }
    let documents = source
        .documents
        .iter()
        .map(|d| (d.document_id.as_str(), d))
        .collect::<BTreeMap<_, _>>();
    let mut ids = BTreeSet::new();
    let mut evidence = Vec::new();
    for span in &source.spans {
        let document = documents
            .get(span.document_id.as_str())
            .ok_or_else(|| invalid("Span has no original source document."))?;
        if !is_digest(&document.content_hash)
            || !is_digest(&span.text_hash)
            || digest(span.quote.as_bytes()) != span.text_hash
        {
            return Err(invalid("Source version or exact quote digest is invalid."));
        }
        if !ids.insert(span.span_id.clone()) {
            return Err(invalid("Duplicate semantic or evidence identity."));
        }
        evidence.push(json!({"id":span.span_id,"sourceId":document.document_id,"sourceVersion":document.content_hash,"sourceHash":document.content_hash,"quote":span.quote,"quoteHash":span.text_hash,"uri":document.uri,"anchorStatus":"exact","stance":"supports"}));
    }
    let spans = ids.clone();
    let mut normalize = |values: &[Value]| -> Result<Vec<Value>, CompilerError> {
        values.iter().map(|value| {
            let mut record = value.as_object().cloned().ok_or_else(|| invalid("Cognitive draft records must be objects."))?;
            let id = record.get("id").and_then(Value::as_str).ok_or_else(|| invalid("Missing explicit semantic record identity."))?;
            if !ids.insert(id.to_owned()) { return Err(invalid("Duplicate semantic or evidence identity.")); }
            if record.get("reviewState").is_some_and(|v| v != "proposed") || record.contains_key("reviewDecision") || record.contains_key("reviewNote") { return Err(invalid("Draft interchange accepts proposed records only; existing review gates remain required.")); }
            record.insert("reviewState".to_owned(), json!("proposed"));
            let provenance = record.get("provenance").and_then(Value::as_object).ok_or_else(|| invalid("Explicit author/model/import provenance is required; the compiler never invents authors."))?;
            if provenance.contains_key("reviewedBy") || provenance.contains_key("reviewedAt") { return Err(invalid("Draft records cannot claim an existing review stamp.")); }
            let source_refs = provenance.get("sourceRefs").and_then(Value::as_array).ok_or_else(|| invalid("Explicit evidence span references are required, including an empty list for undocumented recollection."))?;
            for reference in source_refs {
                if !reference.as_str().is_some_and(|r| spans.contains(r)) { return Err(invalid("Unknown source span anchor in cognitive draft.")); }
            }
            if let Some(refs) = record.get("sourceRefs").and_then(Value::as_array) {
                for reference in refs { if !reference.as_str().is_some_and(|r| spans.contains(r)) { return Err(invalid("Unknown statement evidence span anchor.")); } }
            }
            Ok(Value::Object(record))
        }).collect()
    };
    let nodes = normalize(&request.nodes)?;
    let statements = normalize(&request.statements)?;
    let knowledge = normalize(&request.knowledge)?;
    Ok(
        json!({"schemaVersion":"cognitive-capsule/vNext","capsuleId":request.capsule_id,"version":request.version,"profile":request.profile,"nodes":nodes,"statements":statements,"evidence":evidence,"knowledge":knowledge,"dependencies":request.dependencies,"rights":{"mode":"private"}}),
    )
}

struct TemporaryDraft(PathBuf);
impl Drop for TemporaryDraft {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// Delegate native ZIP serialization and complete semantic validation to the shared PRAXIS runtime.
pub fn write_native_cognitive_capsule(
    draft: &Value,
    praxis_root: &Path,
    output: &Path,
    allow_write: bool,
) -> Result<NativeCognitiveReceipt, CompilerError> {
    if !allow_write {
        return Err(invalid("Native draft export requires --allow-write."));
    }
    if draft.get("schemaVersion").and_then(Value::as_str) != Some("cognitive-capsule/vNext") {
        return Err(invalid("Unsupported native cognitive schema."));
    }
    if output.extension().and_then(|v| v.to_str()) != Some("capsule")
        || output
            .components()
            .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(invalid("Output must be a safe new .capsule path."));
    }
    let output = if output.is_absolute() {
        output.to_path_buf()
    } else {
        std::env::current_dir()?.join(output)
    };
    for ancestor in output.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(meta) if meta.file_type().is_symlink() => {
                return Err(invalid("Native output cannot traverse symlinks."))
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    if output.exists() {
        return Err(invalid(
            "Native artifacts are immutable; output already exists.",
        ));
    }
    let root = fs::canonicalize(praxis_root)?;
    // Windows canonical paths use a verbatim prefix that JS module loaders do not consistently accept.
    #[cfg(windows)]
    let root = PathBuf::from(
        root.to_string_lossy()
            .strip_prefix("\\\\?\\")
            .unwrap_or(&root.to_string_lossy())
            .to_string(),
    );
    let cli = root.join("cli/praxis.ts");
    let launcher = root.join("node_modules/tsx/dist/cli.mjs");
    if !cli.is_file() || !launcher.is_file() {
        return Err(invalid(
            "Explicit PRAXIS checkout requires cli/praxis.ts and installed tsx.",
        ));
    }
    let draft_bytes = serde_json::to_vec(draft)?;
    if draft_bytes.len() > 16 * 1024 * 1024 {
        return Err(invalid("Draft byte budget exceeded."));
    }
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| invalid("Cannot create draft timestamp."))?
        .as_nanos();
    let temporary_path = std::env::temp_dir().join(format!(
        "dialectica-cognitive-{}-{nonce}.json",
        std::process::id()
    ));
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temporary_path)?;
    let temporary = TemporaryDraft(temporary_path);
    file.write_all(&draft_bytes)?;
    drop(file);
    let result = Command::new("node")
        .current_dir(&root)
        .arg(launcher)
        .arg(cli)
        .arg("capsule")
        .arg("pack")
        .arg(&temporary.0)
        .arg("--output")
        .arg(&output)
        .arg("--allow-write")
        .env_remove("PRAXIS_SESSION")
        .env_remove("PRAXIS_AUTH_ORIGIN")
        .output()?;
    if !result.status.success() {
        return Err(invalid("Shared PRAXIS schema/graph/native package validation rejected the draft; no canonical writes occurred."));
    }
    let metadata = fs::metadata(&output)?;
    if metadata.len() > 16 * 1024 * 1024 {
        return Err(invalid("Native archive byte budget exceeded."));
    }
    let bytes = fs::read(&output)?;
    Ok(NativeCognitiveReceipt {
        archive_path: output,
        archive_digest: digest(&bytes),
        schema_version: "cognitive-capsule/vNext".to_owned(),
        status: "draft-only".to_owned(),
        validated_by: "praxis-shared-runtime".to_owned(),
        canonical_writes: false,
    })
}
