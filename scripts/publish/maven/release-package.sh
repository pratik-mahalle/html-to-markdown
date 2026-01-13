#!/usr/bin/env bash
set -euo pipefail

mvn -f packages/java/pom.xml -Ppublish -DskipTests --batch-mode --no-transfer-progress -Dgpg.executable=/usr/bin/gpg2 clean deploy
