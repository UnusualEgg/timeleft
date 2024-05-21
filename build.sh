#!/bin/bash
set -e
wasm-pack build
cd timeleft_page
npm run build
cd ..
