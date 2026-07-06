# Hosted DIALECTICA MCP on Cloud Run

This runbook deploys the DIALECTICA MCP server as a Cloud Run service and keeps
the local Codex stdio MCP path available.

## What Runs

- Service: `dialectica-mcp`
- Endpoint: `/mcp`
- Transport: Streamable HTTP style JSON-RPC POST
- Health: `/health`
- Auth: bearer token from Secret Manager
- Workspace: `DIALECTICA_MCP_WORKSPACE`, defaulting to `/tmp/dialectica-mcp-workspace`

The hosted surface avoids client-local path assumptions. A Codex instance uploads
source text with `dialectica_upload_sources`, receives a `build_id`, and then
builds the capsule with `dialectica_build_uploaded_capsule`.

## One-Time Google Cloud Setup

Set the project and region:

```powershell
$env:GOOGLE_CLOUD_PROJECT = gcloud config get-value project
$env:DIALECTICA_REGION = "us-central1"
```

Create the Artifact Registry repository if it does not already exist:

```powershell
gcloud artifacts repositories describe dialectica --location $env:DIALECTICA_REGION
gcloud artifacts repositories create dialectica --repository-format docker --location $env:DIALECTICA_REGION --description "DIALECTICA containers"
```

Create the bearer-token secret. Do not commit or print the token:

```powershell
$token = Read-Host -AsSecureString "DIALECTICA MCP bearer token"
$bstr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($token)
$plain = [Runtime.InteropServices.Marshal]::PtrToStringAuto($bstr)
$plain | gcloud secrets create dialectica-mcp-bearer-token --data-file=-
[Runtime.InteropServices.Marshal]::ZeroFreeBSTR($bstr)
Remove-Variable plain
```

Grant the Cloud Run runtime service account access to the secret if your project
does not use the default service account permissions:

```powershell
$projectNumber = gcloud projects describe $env:GOOGLE_CLOUD_PROJECT --format "value(projectNumber)"
gcloud secrets add-iam-policy-binding dialectica-mcp-bearer-token --member "serviceAccount:$projectNumber-compute@developer.gserviceaccount.com" --role "roles/secretmanager.secretAccessor"
```

## Deploy

From the repo root:

```powershell
$shortSha = git rev-parse --short HEAD
gcloud builds submit --config cloudbuild.dialectica-mcp.yaml --substitutions "_REGION=$env:DIALECTICA_REGION,_SERVICE=dialectica-mcp,_REPOSITORY=dialectica,_IMAGE_TAG=$shortSha"
```

Read the deployed URL:

```powershell
$url = gcloud run services describe dialectica-mcp --region $env:DIALECTICA_REGION --format "value(status.url)"
```

## Smoke Test

Health does not require bearer auth:

```powershell
Invoke-RestMethod "$url/health"
```

MCP calls require the bearer token:

```powershell
$token = Read-Host -AsSecureString "DIALECTICA MCP bearer token"
$bstr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($token)
$plain = [Runtime.InteropServices.Marshal]::PtrToStringAuto($bstr)
$body = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"codex","version":"1"}}}'
Invoke-RestMethod "$url/mcp" -Method Post -Headers @{ Authorization = "Bearer $plain" } -ContentType "application/json" -Body $body
[Runtime.InteropServices.Marshal]::ZeroFreeBSTR($bstr)
Remove-Variable plain
```

## Codex Connection

Local trusted Codex sessions can keep using stdio:

```toml
[mcp_servers.dialectica]
command = "cargo"
args = ["run", "-p", "dialectica-mcp", "--"]
cwd = "C:\\Users\\giuli\\A3_DIALECTICAbyTACITUS_v3"
```

For Codex builds that support remote Streamable HTTP MCP servers, configure the
Cloud Run endpoint with an authorization header:

```toml
[mcp_servers.dialectica-cloud]
transport = "streamable_http"
url = "https://YOUR-CLOUD-RUN-URL/mcp"
headers = { Authorization = "Bearer ${DIALECTICA_MCP_BEARER_TOKEN}" }
```

If the active Codex build only supports command-based MCP entries, use a small
local bridge process that reads `DIALECTICA_MCP_BEARER_TOKEN` from the
environment and forwards JSON-RPC messages to the Cloud Run `/mcp` endpoint.

## Hosted Capsule Workflow

1. `tools/call` `dialectica_upload_sources`
2. `tools/call` `dialectica_build_uploaded_capsule`
3. `tools/call` `dialectica_inspect_capsule`
4. `tools/call` `dialectica_validate_capsule`
5. `tools/call` `dialectica_export_praxis_pack`

Hosted builds still create draft or assisted capsule artifacts. PRAXIS promotion
remains an explicit human review path.
