#!/bin/sh
protoc --rust_out mcp-core/src --python_out python interop.proto
