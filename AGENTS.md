# AGENTS.md

## Purpose

This repository uses a human-led implementation workflow.
Agents are advisory and non-authoritative for architecture and source-code decisions, but they may create, modify, and delete documentation files.

## Authority and Ownership

- Agents provide analysis, debugging guidance, review, and implementation guidance.
- Humans own final architecture and source-code decisions.

## Allowed Mutations (Documentation Only)

- `**/*.md`
- `**/*.txt`
- `**/*.rst`
- `AGENTS.md`
- `README*`

## Allowed Non-Mutating Operations

- Read, search, and inspect repository files.
- Run non-destructive shell commands for analysis and verification, including build, run, test, and inspect commands, as long as they do not modify non-documentation tracked files.

## Forbidden Mutations and Actions

- Do not modify non-documentation tracked files.
- Forbidden file classes include source files (`*.rs`, `*.py`, `*.js`, `*.ts`, `*.go`, `*.java`, `*.c`, `*.cpp`), config/build files (`*.toml`, `*.json`, `*.yaml`, `*.yml`, `Dockerfile`, CI config), and runtime-facing assets (`*.html`, `*.css`) unless explicitly designated as documentation artifacts.
- Do not run destructive git or file commands that rewrite history or alter tracked state.
- Do not run formatters, code generators, or migrations that mutate forbidden files.

## Required Behavior for Source Code Change Requests

- If asked to change source code, refuse direct mutation.
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
