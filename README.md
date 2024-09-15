# Mobi-Light

## Introduction

<p align="center">
<img src="./blgo2.png" width="148" style="border-radius: 55%;"/>
</p>

- This repository aims to run [Helios](https://github.com/a16z) light client on Android via JNI.
- This repository is a preliminary project for a universal light client project: a cross-platform light client library for all EVM-based chains (including L1 and L2 chains, such as Ethereum Mainnet, BSC, Tron, Scroll, and MegaETH).
- This repository is still under development and is not ready for any production-level projects.

## TODO

### High Priority
- [ ] Task 1: Fix: consensus client constructing failed problem: refactor Helios-Rust
    - [ ] Sub-Task 1: Feat: decouple the consensus client create and sync fucntion.
- [ ] Task 2: Feat: JNI Interface for Android(Rust + Kotlin)
- [ ] Task 3: Feat: Support getting block data via any block number. The current Helios library seems to only support the Latest block and Finalized block based on latest epoch.
    - [ ] Sub-Task 1: Feat: We need modify and add more functions to Helios consensus-inner part.
    - [ ] Sub-Task 2: Docs: We will write a detailed blog about the mechanism about how helios works.
- [ ] Task 4: Automation: Auto Sync the latest checkpoints
- [ ] Task 5: High-availability: how to guarantee the high avaliable network connection via weak mobile net env
## Current Problem
Helios is a powerful library that is still under development. There are many unexpected issues when using it as an external library for Android. Here are several issues we have faced:

### The programs will be crashed when generating a new Client.

1. Inappropriate default absolute address. This issue causes by the following code snippets:  
- https://github.com/a16z/helios/blob/master/client/src/client.rs#L109-L117
- https://github.com/a16z/helios/blob/master/config/src/networks.rs#L56

    **Trick Solution**
    
    This problem will lead app crashing issue. Here is a trick solution to avoid app-carsh. Set the default `data_dir = None`

    ```
    #[cfg(not(target_arch = "wasm32"))]
     data_dir: None,
    ..std::default::Default::default()
    ```
2. Failed in constructing new Node(ConsensusClient). This failure happens in the following code snippet:
- https://github.com/a16z/helios/blob/master/client/src/node.rs#L34-L35

3. Client synced failed. `client.wait_synced().await;`
The client syncing process is killed by JVM for unknown reasons.