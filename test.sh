#!/bin/bash

# CrossChain Registry Test Runner
# Convenience script to run tests from project root

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$SCRIPT_DIR/test"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Show header
echo -e "${CYAN}CrossChain Registry Test Suite${NC}"
echo -e "${CYAN}=============================${NC}"
echo

# Check if test directory exists
if [[ ! -d "$TEST_DIR" ]]; then
    echo -e "${RED}Error: Test directory not found at $TEST_DIR${NC}"
    exit 1
fi

# Show available commands
show_help() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo
    echo "Quick Commands:"
    echo "  quick           Run quick validation tests"
    echo "  test [TYPE]     Run comprehensive test suite"
    echo "  data            Generate test data"
    echo "  clean           Clean up test data"
    echo
    echo "Test Types (for 'test' command):"
    echo "  all             Run all test suites (default)"
    echo "  crud            CRUD operation tests"
    echo "  community       Community validation tests"
    echo "  verification    Verification system tests"
    echo "  errors          Error handling tests"
    echo "  advanced        Advanced feature tests"
    echo "  performance     Performance tests"
    echo "  enhanced        Enhanced features tests (Stage 2)"
    echo "  security        Security and XSS protection tests"
    echo "  monitoring      Monitoring integration tests"
    echo
    echo "Test Options (for 'test' command):"
    echo "  --generate-data Generate test data before running"
    echo "  --performance   Include performance benchmarks"
    echo "  --cleanup       Clean up after tests"
    echo
    echo "Examples:"
    echo "  $0 quick                    # Quick validation"
    echo "  $0 test                     # Run all tests"
    echo "  $0 test --generate-data     # Generate data and run all tests"
    echo "  $0 test crud                # Run CRUD tests only"
    echo "  $0 enhanced                 # Run enhanced features tests"
    echo "  $0 security                 # Run security tests only"
    echo "  $0 data                     # Generate test data"
    echo "  $0 clean                    # Clean test data"
    echo
    echo "Files:"
    echo "  Test scripts are located in: ./test/"
    echo "  Test logs will be in: ./test/logs/"
    echo "  Test reports: ./test/logs/test_report_*.html"
}

# Parse command
case "${1:-help}" in
    "quick"|"q")
        echo -e "${BLUE}Running quick validation tests...${NC}"
        "$TEST_DIR/quick_test.sh"
        ;;
    
    "test"|"t")
        shift  # Remove 'test' from arguments
        echo -e "${BLUE}Running comprehensive test suite...${NC}"
        "$TEST_DIR/run_tests.sh" "$@"
        ;;
    
    "enhanced"|"e")
        echo -e "${BLUE}Running enhanced features test suite...${NC}"
        "$TEST_DIR/run_enhanced_tests.sh"
        ;;
    
    "security"|"s")
        echo -e "${BLUE}Running security tests...${NC}"
        "$TEST_DIR/test_enhanced_security.sh"
        ;;
    
    "monitoring"|"m")
        echo -e "${BLUE}Running monitoring tests...${NC}"
        "$TEST_DIR/test_monitoring.sh"
        ;;
    
    "data"|"d")
        echo -e "${BLUE}Generating test data...${NC}"
        "$TEST_DIR/test_data_generator.sh" generate
        ;;
    
    "clean"|"c")
        echo -e "${YELLOW}Cleaning test data...${NC}"
        "$TEST_DIR/test_data_generator.sh" clean
        ;;
    
    "logs"|"l")
        echo -e "${BLUE}Opening test logs directory...${NC}"
        if [[ -d "$TEST_DIR/logs" ]]; then
            ls -la "$TEST_DIR/logs/"
            echo
            echo "Latest test report:"
            find "$TEST_DIR/logs" -name "test_report_*.html" -type f -exec ls -t {} + | head -1
        else
            echo "No test logs found. Run tests first."
        fi
        ;;
    
    "status"|"s")
        echo -e "${BLUE}Test environment status:${NC}"
        echo
        
        # Check dfx status
        if dfx ping > /dev/null 2>&1; then
            echo -e "  DFX Network: ${GREEN}Running${NC}"
        else
            echo -e "  DFX Network: ${YELLOW}Not running${NC}"
        fi
        
        # Check canister deployment
        if dfx canister call CrossChainRegistry_backend get_company_count > /dev/null 2>&1; then
            count=$(dfx canister call CrossChainRegistry_backend get_company_count 2>/dev/null | grep -o '[0-9]*' | head -1)
            echo -e "  Backend Canister: ${GREEN}Deployed${NC} ($count companies)"
        else
            echo -e "  Backend Canister: ${YELLOW}Not deployed${NC}"
        fi
        
        # Check test logs
        if [[ -d "$TEST_DIR/logs" ]]; then
            log_count=$(ls "$TEST_DIR/logs" 2>/dev/null | wc -l)
            echo -e "  Test Logs: ${GREEN}$log_count files${NC}"
        else
            echo -e "  Test Logs: ${YELLOW}None${NC}"
        fi
        ;;
    
    "help"|"h"|"--help"|"-h")
        show_help
        ;;
    
    *)
        echo -e "${YELLOW}Unknown command: $1${NC}"
        echo
        show_help
        exit 1
        ;;
esac