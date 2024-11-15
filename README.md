# Token Wallet

## Overview

The **Token Wallet** is a secure digital wallet built on the Internet Computer Protocol (ICP) blockchain. It allows users to send and receive IRCRC2 tokens seamlessly while ensuring high security and performance.

## Features

- **Send Tokens**: Users can send tokens to other addresses easily.
- **Receive Tokens**: The wallet can receive tokens and update balances accordingly.
- **Balance Display**: Users can view their current token balance.
- **Unit Testing**: Comprehensive tests ensure the functionality of sending and receiving tokens.

## Getting Started

### Prerequisites

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- [DFX](https://internetcomputer.org/docs/current/developers-guide/install-upgrade-dfx/) installed for deploying on ICP.
- A local ICP test network running.

### Installation

1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd token_wallet

Start the local ICP network:
bash
dfx start --background

Deploy the wallet:
bash
dfx deploy

Running Tests
To run the unit tests, execute:
bash
dfx test

Usage
Once deployed, you can interact with the wallet through its API or integrate it into your frontend application.
Example API Calls
Send Tokens:
Call the send_tokens function with recipient address and amount.
Receive Tokens:
Call the receive_tokens function with amount.
Get Balance:
Call the get_balance function to retrieve current balance.
