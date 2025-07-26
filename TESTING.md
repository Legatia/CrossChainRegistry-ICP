# CrossChain Registry Testing Guide

This document provides a quick overview of the testing infrastructure for the CrossChain Registry project.

## 🚀 Quick Testing Commands

```bash
# Quick validation (recommended first step)
./test.sh quick

# Run all tests with data generation
./test.sh test --generate-data

# Run specific test types
./test.sh test crud           # CRUD operations only
./test.sh test community      # Community validation only
./test.sh test verification   # Verification system only

# Utility commands
./test.sh status              # Check test environment status
./test.sh data                # Generate test data
./test.sh clean               # Clean test data
./test.sh logs                # View test logs
```

## 📁 Project Structure

```
CrossChainRegistry/
├── test.sh                   # 🎯 Main test runner (start here)
├── test/                     # 📂 All testing files
│   ├── run_tests.sh          # Master test orchestrator
│   ├── test_backend.sh       # Core test engine (46+ tests)
│   ├── quick_test.sh         # Quick validation (10 tests)
│   ├── test_data_generator.sh # Test data generation
│   ├── test_config.json      # Test configuration
│   ├── TEST_AUTOMATION_README.md # Detailed documentation
│   └── logs/                 # Test execution logs
└── src/CrossChainRegistry_backend/ # Backend source code
```

## ✅ Test Coverage

- **46+ individual tests** covering all 47 API endpoints
- **5 test categories**: CRUD, Community Validation, Verification, Errors, Advanced
- **3 performance benchmarks** for key operations
- **Automated HTML reports** and detailed logging

## 🛠 Test Categories

| Category | Tests | What it covers |
|----------|-------|----------------|
| **CRUD** | 7 tests | Company creation, retrieval, updates, listing, search |
| **Community** | 12 tests | Endorsements, testimonials, vouches, reputation |
| **Verification** | 10 tests | Domain verification, cross-chain challenges |
| **Errors** | 6 tests | Invalid inputs, authorization failures |
| **Advanced** | 8 tests | Filtering, pagination, complex queries |
| **Performance** | 3 benchmarks | Speed testing and concurrent operations |

## 📊 Example Test Run

```bash
$ ./test.sh quick
CrossChain Registry Test Suite
=============================

Running quick validation tests...
🚀 Running Quick Backend Validation Tests
=========================================
✅ Test 1: Get company count
✅ Test 2: Create test company  
✅ Test 3: Get created company
✅ Test 4: Add testimonial
✅ Test 5: Add community vouch
✅ Test 6: Get community validation data
✅ Test 7: Search companies
✅ Test 8: Get registry statistics
✅ Test 9: Create domain verification challenge
✅ Test 10: Create cross-chain verification challenge

🎉 Quick validation tests completed successfully!
```

## 🔧 Prerequisites

```bash
# 1. Install DFX (if not already installed)
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# 2. Start local ICP network
dfx start --background

# 3. Deploy canisters (automatic in tests, or manual)
dfx deploy

# 4. Run tests
./test.sh quick
```

## 📋 Test Results

### Test Logs Location
- **Main logs**: `test/logs/`
- **HTML reports**: `test/logs/test_report_*.html`
- **Performance data**: `test/logs/benchmark_*.txt`

### Status Checking
```bash
# Check environment status
./test.sh status

# Output:
# DFX Network: Running ✅
# Backend Canister: Deployed ✅ (7 companies)
# Test Logs: 5 files ✅
```

## 🚨 Troubleshooting

| Issue | Solution |
|-------|----------|
| `DFX is not running` | Run `dfx start --background` |
| `Canister not deployed` | Run `dfx deploy` |
| `Permission denied` | Run `chmod +x test.sh` |
| `Tests failing` | Check `./test.sh status` and logs |

## 📚 Detailed Documentation

For comprehensive testing documentation, see: [`test/TEST_AUTOMATION_README.md`](test/TEST_AUTOMATION_README.md)

---

**Quick Start**: Run `./test.sh quick` to validate everything is working! 🎉