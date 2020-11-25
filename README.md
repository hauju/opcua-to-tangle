# Overview

The idea of the project is to create a connectivity solution to combine the advantages of OPC UA and distributed ledger technology like IOTA Streams The opcua-to-tangle project consists of three components.

1. The I2TH OPC UA Sensors Server is just an example server that reveals some OPC UA nodes we can connect to.​ In theory, any OPC UA server can be used.
1. The I2TH OPC UA Streams Gateway connects e.g. to the OPC UA Sensors Server, fetch the sensor data and publish them to the tangle. The channel address is also provided via OPC UA. 
1. The I2TH OPC UA Streams Subscriber connects to the Tangle, obtains the latest data, and makes it available via OPC UA.

Further information can be found in the [Wiki](https://github.com/hauju/opcua-to-tangle/wiki).

# References

The project are based on the OPC UA implementation from [locka99](https://github.com/locka99/opcua) and contains samples from the [IOT2TANGLE](https://github.com/iot2tangle) repositories.

# License

The code is licenced under [Apache-2.0](https://opensource.org/licenses/Apache-2.0).

# Setup

The code is build and tested under Windows with the Windows Subsytem for Linux (WSL2) Environment. The Repository contains a devcontainer which is the simplest way to build the project. It contains all dependecies, you only need the following tools installed and configured.

- Docker
- Visual Studio Code
- [ms-vscode-remote.vscode-remote-extensionpack](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack)


# Getting started

For simplicity, the instructions are only for running the components on the local PC btw. in the docker environment. Look at the wiki for cross-compiling and execution on a Raspberry PI. Run the servers in the right order to follow the sample.

1. Open the workspace with vscode and than reopen the folder in the devcontainer.
1. Open a terminal and run the OPC UA Sensors Server.
    ```bash
    cd opcua-sensors-server && cargo run --release
    ```

1. Open an other terminal and run the OPC UA Streams Gateway.
    ```bash
    # Add the OPC UA Server url you want to connect e.g. "opc.tcp://localhost:4855"
    cd opcua-streams-gateway && cargo run -- <url>
    ```

1. Open another terminal and run the OPC UA Streams Subscriber.
    ```bash
    # Add the channel published by the OPC UA Streams Gateway
    cd opcua-streams-subscriber && cargo run --release -- <channel>
    ```
1. Now you can connect to the servers with an OPC UA Client, trust the certificates and discover the nodes. I usually use the [UaExpert](https://www.unified-automation.com/products/development-tools/uaexpert.html), which is free to use but require an registration.


# Next steps and suggestions for improvement

- Separating the examples and libraries.
- Creating crates from the libraries.
- Creating the docs for the libraries.
- Adding CI / CD workflow.
- Using keepy.
- Implementing the channel exchange between gateway and subscriber via OPC UA.
- Implementing the OPC UA Pub / Sub Model for the opcua-streams-gateway.
- Creating a standardized OPC UA Information Model for IOT2Tangle and all used types.
- Add the Historical Access functionality.
