# CrossChain Registry

A decentralized trust platform for Web3 companies on the Internet Computer Protocol (ICP).

## What We're Building

**CrossChain Registry** is a comprehensive trust verification system that enables Web3 companies to build credible, transparent profiles across multiple blockchain ecosystems. Think of it as a "Web3 Better Business Bureau" with built-in security monitoring.

### 🔍 **Core Features**
- **Company Verification**: Multi-platform identity verification (Twitter, Discord, GitHub, etc.)
- **Cross-Chain Presence**: Track and verify blockchain addresses and smart contracts
- **Community Validation**: Peer endorsements, testimonials, and reputation scoring
- **Security Monitoring**: Real-time threat detection and proof integrity monitoring
- **Trust Transparency**: Permanent audit trails with community oversight

### 🛡️ **Security & Monitoring System**
- **Automated Monitoring**: Continuous verification that social media proofs remain active
- **Community Reporting**: Decentralized flagging of suspicious behavior
- **Security Event Logging**: Comprehensive logging of all security-related activities
- **Rate Limiting**: Multi-tier protection against abuse and spam
- **Trust Scoring**: Dynamic reputation system based on verification integrity

Companies that delete their verification posts after approval are automatically flagged, creating accountability and preventing gaming of the system.

## 🚀 **Quick Start**

```bash
# Start local IC replica
dfx start --background

# Deploy the registry
dfx deploy

# Run tests
./test.sh quick
```

## 📁 **Project Structure**

```
CrossChainRegistry/
├── src/CrossChainRegistry_backend/     # Rust canister (main logic)
│   ├── src/
│   │   ├── monitoring.rs              # Security & monitoring system
│   │   ├── verification.rs            # Identity verification logic
│   │   ├── community.rs               # Community validation features
│   │   └── types.rs                   # Data structures
├── src/CrossChainRegistry_frontend/    # React frontend
├── test/                              # Comprehensive test suite
├── docs/                              # Project documentation
│   ├── MONITORING_SYSTEM.md           # Security system documentation
│   ├── MONITORING_TEST_RESULTS.md     # Test verification results
│   └── README.md                      # Documentation index
├── README.md                          # This file
└── TESTING.md                         # Testing documentation
```

## 🔗 **Key Resources**

- **📖 Documentation**: See [`/docs`](./docs/) directory for comprehensive project documentation
- **🖥️ Frontend**: Access deployed app via DFX-provided URL
- **⚙️ Backend API**: Test via Candid UI or `dfx canister call`
- **🛡️ Monitoring**: Real-time security event logging and community reporting
- **🧪 Testing**: Run `./test.sh` for comprehensive validation

**Status**: ✅ Fully operational with security monitoring system deployed and tested.
