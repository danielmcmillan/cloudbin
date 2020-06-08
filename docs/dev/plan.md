# Plan

Platform for user-generated applications provided as WebAssembly binaries.
Applications can be run in the web browser, with the ability to receive user input and output
graphics and audio.
Applications can also be run server-side, with the ability to communicate with clients running in
the web browser.

## Stages

### Stage 1

Minimum POC for running application in the browser.

- [x] Run wasm binary in browser
- [x] Provide simple text buffer API
- [ ] SPA with wasm binary downloaded from server

### Stage 2

Application management

- [ ] Upload application as archive
- [ ] Application manifest file used to determine interfaces provided to binary
- [ ] List and view details of uploaded binaries

### Stage 3

Slightly enhanced interfaces for client

- [ ] Provide simple API for user input
- [ ] Provide text-based graphics API

### Stage 4

Server binary

- [ ] Load, compile, and run binaries on server
- [ ] Provide client API for interfacing with server: send and receive message
- [ ] Provide server API for interfacing with clients: send, broadcast and receive message
- [ ] No grouping of clients - all clients are in the same session
- [ ] No continuity - after disconnect, client may be seen as new participant from server
- [ ] User provides name of session before starting client (no security or session lifetime)
- [ ] Provide server API for some simple per-session storage
- [ ] Server side binary is ephemeral - possibly being downloaded, compiled + executed for every
      individual message from client

### Beyond

- Better graphics API
- Users
- Session management:
  - Users can create or join sessions
  - App defines allowed number of people per session
  - Session list + participants updates in real time
  - App defines whether game starts immediately or participants first stay in waiting room
  - App defines whether people can join mid-session
  - Sessions can be invite-only
- Improved latency: keep server binary loaded
- Client audio

## Goals

- Keep cloud-provider specific functionality abstracted away as much as possible
- Good browser support is non-goal. Use latest experimental features.

### Security

- Webassembly binaries have access to a small controlled set of APIs
- APIs and priviledges required are defined in a manifest file included with the binary
- Complete security is non-goal. Some necessary things are low priority (e.g. limiting CPU time and
  memory usage)

### Extensible set of interfaces

Interfaces provided to the untrusted binaries will initially be very simple.

- provide more capabilities or completely change interface without breaking compatability with
  existing binaries
- allow choice of which interfaces should be provided by defining this in the manifest file
- some APIs would be incompatible (e.g. only 1 graphics API can be used)
- interfaces are versioned
- interfaces define both symbols expected to be exported and symbols that will be made available at
  runtime
