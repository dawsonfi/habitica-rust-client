#!/bin/bash
set -ev
if [[ "${TRAVIS_COMMIT_MESSAGE}" = *"Release"* ]]; then
    cargo publish
fi
