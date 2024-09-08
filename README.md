# helios-jni-rs

## Introduction

- This repository aims to run Helios light client on Android via JNI.
- This repository is a preliminary project for a universal light client project: a cross-platform light client library for all EVM-based chains (including L1 and L2 chains, such as Ethereum Mainnet, BSC, Tron, Scroll, and MegaETH).
- This repository is still under development and is not ready for any production-level projects.

## TODO

### High Priority
- [ ] Task 1: Fix: consensus client constructing failed problem: refactor Helios-Rust
    - [ ] Sub-Task 1: Feat: decouple the consensus client create and sync fucntion.
- [ ] Task 2: Feat: JNI Interface for Android(Rust + Kotlin)

## Current Problem
Helios is a powerful library that is still under development. There are many unexpected issues when using it as an external library for Android. Here are several issues we have faced:

### The programs will be crashed when generating a new Client.

**Why**
1. Inappropriate default absolute address. This issue causes by the following code snippets:  
- https://github.com/a16z/helios/blob/master/client/src/client.rs#L109-L117
- https://github.com/a16z/helios/blob/master/config/src/networks.rs#L56


2. Failed in constructing new Node(ConsensusClient). This failure happens in the following code snippet:
- https://github.com/a16z/helios/blob/master/client/src/node.rs#L34-L35

3. Client synced failed. `client.wait_synced().await;`
The client syncing process is killed by JVM for unknown reasons.