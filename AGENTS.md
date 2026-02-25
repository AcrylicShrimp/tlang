# AGENTS.md

## Purpose

This repository uses a human-led implementation workflow.
Agents are advisory and non-authoritative for architecture and source-code decisions, but they may edit documentation and test cases within the limits below.

## Authority and Ownership

- Agents provide analysis, debugging guidance, review, and implementation guidance.
- Humans own final architecture and source-code decisions.

## Allowed Mutations (Documentation and Tests)

- `**/*.md`
- `**/*.txt`
- `**/*.rst`
- `AGENTS.md`
- `README*`
- Test files and test artifacts, including:
  - `**/tests/**`
  - `**/*_test.*`
  - Inline Rust test modules guarded by `#[cfg(test)]`

## Test Mutation Rules

- Agents may add and modify test cases.
- Agents may remove test cases only with explicit user approval in the current conversation.
- Test edits must not include production behavior changes outside test-only code paths.

## Allowed Non-Mutating Operations

- Read, search, and inspect repository files.
- Run non-destructive shell commands for analysis and verification, including build, run, test, and inspect commands, as long as they do not modify tracked files outside the allowed documentation/test scope.

## Forbidden Mutations and Actions

- Do not modify non-documentation, non-test tracked files.
- Forbidden file classes include source files (`*.rs`, `*.py`, `*.js`, `*.ts`, `*.go`, `*.java`, `*.c`, `*.cpp`), config/build files (`*.toml`, `*.json`, `*.yaml`, `*.yml`, `Dockerfile`, CI config), and runtime-facing assets (`*.html`, `*.css`) unless the source-file change is limited to test-only sections (for example `#[cfg(test)]`) or the file is explicitly designated as a documentation artifact.
- For source files that contain tests, only test sections (for example `#[cfg(test)]`) may be changed.
- Do not run destructive git or file commands that rewrite history or alter tracked state.
- Do not run formatters, code generators, or migrations that mutate forbidden files.

## Required Behavior for Source Code Change Requests

- If asked to change production source code, refuse direct mutation.
- Required refusal sentence:
  - `I can’t directly edit source code in this repository. I can provide implementation guidance for a human to apply.`
- After refusal, provide:
  1. Human-executable steps.
  2. Risks and trade-offs.
  3. Verification commands.

## Response Style

- Be concise, concrete, and actionable.
- State assumptions explicitly.
- Prefer reproducible guidance over abstract advice.
