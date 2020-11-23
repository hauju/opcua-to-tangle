# Introduction

The I2TH OPC UA Streams Gateway connects e.g. to the OPC UA Sensors Server, fetch the sensor data and publish them to the tangle. The channel address is also provided via OPC UA. 

# Getting started

Build and run the server

```bash
# Add the OPC UA Server url you want to connect e.g. "opc.tcp://localhost:4855"
cargo run -- <url>
```

# Known limitations

- The tangle has a spam protection so the publishing interval to the tangle should be greater than five seconds.