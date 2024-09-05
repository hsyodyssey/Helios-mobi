# helios-jni-rs

- This repo aims to run Helios light client on Android via JNI.
- This repo is the pre-project for universal lightclient project.

## TODO

### High Priority
- [ ] Task 1: Fix: consensus client constructing failed problem: refactor Helios-Rust
- [ ] Task 2: Feat: JNI Interface for Android(Rust + Kotlin)

## Current Problem
Helios is a powerful library that still under development. Therefore, there are many unexpected issues when using it.

### The programs will be crashed when generating a new Client.

**Why**
1. Inappropriate default absolute address. This issue causes by the following code snippets: 
- https://github.com/a16z/helios/blob/master/client/src/client.rs#L109-L117
- https://github.com/a16z/helios/blob/master/config/src/networks.rs#L56

Android has stricter rules for file access, which causes this problem.

2. Failed in constructing new Node(ConsensusClient). This failure happens in the following code snippet:
- https://github.com/a16z/helios/blob/master/client/src/node.rs#L34-L35

