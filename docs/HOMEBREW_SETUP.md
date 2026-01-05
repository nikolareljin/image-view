# Homebrew Tap Setup

This document explains how to set up automated Homebrew formula updates for the image-view project.

## Overview

When a new release tag is pushed, the GitHub Actions workflow automatically:
1. Builds tarballs for macOS (ARM64 & x86_64) and Linux (x86_64)
2. Uploads them as GitHub release assets
3. Calculates SHA256 hashes for each platform
4. Updates the Homebrew formula in the tap repository
5. Commits and pushes the changes to the tap repository

## Required GitHub Secrets

Configure these secrets in the image-view repository settings:

### 1. HOMEBREW_TAP_GITHUB_TOKEN

A GitHub Personal Access Token (PAT) with write access to the tap repository.

**How to create:**
1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Give it a descriptive name (e.g., "Homebrew Tap Updates for image-view")
4. Set expiration (recommend: 1 year, then set a calendar reminder to rotate)
5. Select scopes:
   - ✅ `repo` (Full control of private repositories)
6. Click "Generate token"
7. Copy the token immediately (you won't be able to see it again!)

**Add to repository:**
1. Go to image-view repository → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `HOMEBREW_TAP_GITHUB_TOKEN`
4. Value: Paste the token
5. Click "Add secret"

### 2. HOMEBREW_TAP_REPO

The repository name for your Homebrew tap.

**Value:** `nikolareljin/homebrew-tap`

**Add to repository:**
1. Go to image-view repository → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `HOMEBREW_TAP_REPO`
4. Value: `nikolareljin/homebrew-tap`
5. Click "Add secret"

## Workflow Triggers

The workflow runs when you push a tag matching these patterns:
- `v*.*.*` (e.g., v0.5.0, v1.0.0)
- `*.*.*` (e.g., 0.5.0, 1.0.0)
- `v*.*.*-rc` or `*.*.*-rc` (release candidates)
- `v*.*.*-RC` or `*.*.*-RC` (release candidates)

## Creating a Release

To trigger the workflow and update the Homebrew tap:

```bash
# Ensure version is updated in Cargo.toml
git tag v0.6.0
git push origin v0.6.0
```

The workflow will:
1. Build binaries for all platforms
2. Create a GitHub release with tarballs
3. Update the Homebrew formula with new version and SHA256 hashes
4. Push the updated formula to the tap repository

## Verifying the Setup

After configuring the secrets, you can verify the setup by:

1. **Check secrets are configured:**
   - Go to repository Settings → Secrets and variables → Actions
   - Verify both `HOMEBREW_TAP_GITHUB_TOKEN` and `HOMEBREW_TAP_REPO` exist

2. **Test with a release:**
   - Create and push a test tag (e.g., `v0.5.1-test`)
   - Watch the workflow run in the Actions tab
   - Check the workflow logs for the "Validate secrets" step

## Workflow Features

The enhanced workflow includes:

- **Secret Validation:** Checks that required secrets are configured before running
- **SHA Verification:** Validates SHA256 hash format before updating formula
- **Diff Output:** Shows what changes will be committed
- **Formula Validation:** Validates Ruby syntax before committing
- **Retry Logic:** Retries git push up to 3 times if it fails (handles concurrent updates)
- **Step Summary:** Generates a summary with release info and SHA hashes

## Troubleshooting

### Workflow fails with "Missing required secrets"
- Ensure both secrets are configured in the image-view repository
- Check that the token hasn't expired

### Workflow fails during push
- Verify the PAT has `repo` scope
- Check that the token has write access to the homebrew-tap repository
- Ensure the tap repository exists and is accessible

### Formula not updating
- Check the workflow logs in the Actions tab
- Verify the formula file exists at `Formula/image-view.rb` in the tap repository
- Ensure the formula follows the expected format

## Manual Formula Update

If you need to update the formula manually:

```bash
cd /path/to/homebrew-tap
nano Formula/image-view.rb
# Update version and SHA256 hashes
git commit -am "Update image-view to vX.Y.Z"
git push
```

## Installation

Once the tap is updated, users can install image-view with:

```bash
brew tap nikolareljin/tap
brew install image-view
```

Or in one command:

```bash
brew install nikolareljin/tap/image-view
```
