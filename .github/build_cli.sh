#!/usr/bin/env bash
# Author: wswenyue.
#Date & Time: 2024-03-15 19:55:32
#Description: a bash script
ls -alh ./

echo "=============cli:${CLI_NAME}======build begin================"
cargo build --verbose --release -p "${CLI_NAME}" --target "${MATRIX_TARGET}"
echo "=============cli:${CLI_NAME}======build end================"

ls -alh ./

echo "----------------./target/release/-----------------------------"
ls -alh ./target/release/
echo "----------------./target/${MATRIX_TARGET}---------------------"
ls -alh "./target/${MATRIX_TARGET}"
echo "----------------./target/${MATRIX_TARGET}/release-------------"
ls -alh "./target/${MATRIX_TARGET}/release"
echo "--------------------------------------------------------------"

asset_cli_name="${CLI_NAME}-${APP_VERSION}-${MATRIX_TARGET}"
echo "=================pack target binary${MATRIX_TARGET}=====begin============="
if [ "${MATRIX_OS}" = "windows-latest" ]; then
  cli_binary="./target/${MATRIX_TARGET}/release/${CLI_NAME}.exe"
  if [ -e "${cli_binary}" ]; then
    mv "${cli_binary}" "./${CLI_NAME}.exe"
    7z a "${asset_cli_name}.zip" "./${CLI_NAME}.exe"
    echo "ASSET_CLI=${asset_cli_name}.zip" >>$GITHUB_ENV
  else
    echo "file no exists!!! cli_binary :${cli_binary}"
    exit 1
  fi

else
  cli_binary="./target/${MATRIX_TARGET}/release/${CLI_NAME}"
  if [ -e "${cli_binary}" ]; then
    mv "${cli_binary}" "./${CLI_NAME}"
    mv "./target/release/${CLI_NAME}.1" "./${CLI_NAME}.1"
    tar -czf "${asset_cli_name}.tar.gz" "./${CLI_NAME}" "./${CLI_NAME}.1"
    echo "ASSET_CLI=${asset_cli_name}.tar.gz" >>$GITHUB_ENV
  else
    echo "file no exists!!! cli_binary :${cli_binary}"
    exit 1
  fi
fi
echo "=================pack target binary================end===================="

echo "succeed."
