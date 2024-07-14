---

# Nft-mint_vault_swap

---

# NFT Minting, Vault Integration, and Swap

This project demonstrates the creation of non-fungible tokens (NFTs), their integration with a vault system, and the ability to swap NFTs on the Solana blockchain.

## Overview

This application allows users to:

- Mint new NFTs with customizable metadata.
- Store and manage NFTs securely using a vault system.
- Swap NFTs between users.

## Features

- **NFT Minting**: Users can mint new NFTs with unique properties.
- **Vault Integration**: Securely store and manage NFTs using a vault system.
- **NFT Swap**: Exchange NFTs between different accounts.
- **Smart Contract**: Developed using Anchor framework for reliability and security.


## Setup Instructions

### Prerequisites

- Rust programming language and Cargo installed.
- Solana CLI (`solana`) and Anchor CLI (`anchor`) installed.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/luckysitara/Nft-mint_vault_swap.git
   cd Nft-mint_vault_swap
   ```

2. Install dependencies:

   ```bash
   chmod +x install-depen.sh; ./install-depen.sh
   cd programs/nft
   anchor build
   ```

### Deployment

Deploy the program to a localnet or testnet using Anchor CLI:

```bash
anchor deploy
```

### Usage

1. **Initialize the Program**:

   ```bash
   anchor invoke --program-id <program_id> initialize
   ```

2. **Mint NFT**:

   ```bash
   anchor invoke --program-id <program_id> mint_nft --title "My NFT" --description "Description of NFT"
   ```

3. **Vault Operations**:

   Implement vault functionalities to securely store and manage NFTs.

4. **Swap NFTs**:

   Implement swap functionality to exchange NFTs between accounts.

### Additional Features

- **Zero-Knowledge Proofs**: Implement ZK proofs for enhanced privacy and security.
- **Points System**: Integrate a points system for rewarding users based on NFT activities.

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

---

