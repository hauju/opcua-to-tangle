# Introduction

The I2TH OPC UA Streams Subscriber connects to the Tangle, obtains the latest data, and makes it available via OPC UA.

# Getting started

Build and run the server

```bash
# Add the channel published by the OPC UA Streams Gateway
cargo run --release -- <channel>
```

# Known limitations

- Reconnecting to an existing channel with a lot of messages does not work.