param(
    [string]$Region = "us-central1",
    [string]$Service = "dialectica-mcp",
    [string]$Repository = "dialectica",
    [string]$SecretName = "dialectica-mcp-bearer-token",
    [string]$ImageTag = ""
)

$ErrorActionPreference = "Stop"

if (-not $ImageTag) {
    $ImageTag = (git rev-parse --short HEAD).Trim()
}

$project = (gcloud config get-value project 2>$null).Trim()
if (-not $project) {
    throw "No active gcloud project. Run: gcloud config set project PROJECT_ID"
}

gcloud secrets describe $SecretName --project $project *> $null
if ($LASTEXITCODE -ne 0) {
    throw "Missing Secret Manager secret '$SecretName'. Create it before deploying; see docs/HOSTED_MCP_CLOUD_RUN.md."
}

gcloud artifacts repositories describe $Repository --location $Region --project $project *> $null
if ($LASTEXITCODE -ne 0) {
    gcloud artifacts repositories create $Repository `
        --repository-format docker `
        --location $Region `
        --description "DIALECTICA containers" `
        --project $project
}

gcloud builds submit `
    --config cloudbuild.dialectica-mcp.yaml `
    --substitutions "_REGION=$Region,_SERVICE=$Service,_REPOSITORY=$Repository,_IMAGE_TAG=$ImageTag" `
    --project $project

$url = (gcloud run services describe $Service --region $Region --project $project --format "value(status.url)").Trim()
Write-Output "DIALECTICA MCP deployed: $url"
Write-Output "Smoke test: Invoke-RestMethod '$url/health'"
