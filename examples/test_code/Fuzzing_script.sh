#!/bin/bash
cargo fuzz run b_algebra & 
cargo fuzz run b_ifStmt & 
cargo fuzz run b2_ifStmt &
cargo fuzz run b_loop &
cargo fuzz run b_infLoop