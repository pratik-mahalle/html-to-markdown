#!/usr/bin/env bash
set -euo pipefail

# Copy native libraries into resources if java-ffi-artifacts directory exists
if [[ -d "java-ffi-artifacts" ]]; then
	echo "Found java-ffi-artifacts directory, copying native libraries..."
	scripts/publish/java/copy-native-libs.sh java-ffi-artifacts
else
	echo "Warning: java-ffi-artifacts directory not found, native libraries will not be bundled"
fi

mvn -f packages/java/pom.xml -Ppublish -DskipTests --batch-mode --no-transfer-progress clean deploy
