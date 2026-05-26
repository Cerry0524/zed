# Traditional Chinese UI Slice Plan

This document tracks the first Traditional Chinese vertical slice for the
personal `Cerry0524/zed` fork.

## Goal

Make a small, visible, testable portion of the Zed interface render in
Traditional Chinese on this Mac while keeping the fork maintainable enough to
continue translating more surfaces later.

## First Slice

The first implementation slice focuses on high-visibility UI that can be checked
quickly after launching Zed:

- command palette labels and empty-state actions
- settings entry labels that appear before opening deep settings pages
- project panel common actions and tooltips
- common dialog/action labels such as `Copy`, `Ok`, `Retry`, `Dismiss`, and
  `Configure`

## Out of Scope for the First Slice

- full documentation translation
- full AI/Agent text translation
- full debugger text translation
- every error message in the application
- OS installer translations
- changing Zed's product name or command identifiers

## Implementation Direction

Use a minimal Rust localization helper for the first slice instead of replacing
all English string literals directly. This keeps the first slice easy to review
and gives future work a single place to expand terminology.

The helper should:

- support `zh-Hant`, `zh-TW`, and `zh_Hant` style locale tags
- default to Traditional Chinese for this personal fork
- return English text when a phrase is not translated yet
- avoid affecting non-user-facing identifiers and action names

## Verification

Each implementation step should include:

- a focused unit test for locale selection or phrase lookup
- a compile/test command for the touched crate when practical
- a final `git status --short --branch` check before each commit

Manual app verification should check that the command palette, settings entry
points, and project panel show the first translated labels.
