#!/usr/bin/env bash
# Author: wswenyue.
#Date & Time: 2024-03-15 19:55:32
#Description: a bash script
ls -alh ./

echo "=================pack target:${MATRIX_TARGET}======begin========="
asset_target="target-${APP_VERSION}-${MATRIX_TARGET}.tar.gz"
tar -czf "${asset_target}" target
echo "asset_target=${asset_target}" >> $GITHUB_ENV
echo "=================pack target================end=================="
ls -alh "./target/release/"

asset_binary_name="${APP_NAME}-${APP_VERSION}-${MATRIX_TARGET}"
echo "=================pack target binary${MATRIX_TARGET}=====begin============="
if [ "${MATRIX_OS}" = "windows-latest" ]; then
  7z a "${asset_binary_name}.zip" "target/${MATRIX_TARGET}/release/${APP_NAME}.exe"
  echo "asset_binary=${asset_binary_name}.zip" >> $GITHUB_ENV
else
  tar -czf "${asset_binary_name}.tar.gz" "target/${MATRIX_TARGET}/release/${APP_NAME}"
  echo "asset_binary=${asset_binary_name}.tar.gz" >> $GITHUB_ENV
fi
echo "=================pack target binary================end===================="

echo "succeed."