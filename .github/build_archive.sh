#!/usr/bin/env bash
# Author: wswenyue.
#Date & Time: 2024-03-15 19:55:32
#Description: a bash script
ls -alh ./
echo "=============io_tools======build begin================"
cargo build --verbose --release -p io_tools --target "${MATRIX_TARGET}"
echo "=============io_tools======build end================"
echo "=============io_tools_desk.app======build begin================"
cargo tauri build --verbose --target "${MATRIX_TARGET}"
echo "=============io_tools_desk.app======build end================"
ls -alh ./
echo "=================pack target:${MATRIX_TARGET}======begin========="
asset_target="target-${APP_VERSION}-${MATRIX_TARGET}.tar.gz"
tar -czf "${asset_target}" target
echo "asset_target=${asset_target}" >> $GITHUB_ENV
echo "=================pack target================end=================="
echo "----------------./target/release/-----------------------------"
ls -alh ./target/release/
echo "----------------./target/${MATRIX_TARGET}---------------------"
ls -alh "./target/${MATRIX_TARGET}"
echo "----------------./target/${MATRIX_TARGET}/release-------------"
ls -alh "./target/${MATRIX_TARGET}/release"
echo "--------------------------------------------------------------"
asset_binary_name="${APP_NAME}-${APP_VERSION}-${MATRIX_TARGET}"
echo "=================pack target binary${MATRIX_TARGET}=====begin============="
if [ "${MATRIX_OS}" = "windows-latest" ]; then
  binary="./target/${MATRIX_TARGET}/release/${APP_NAME}.exe"
  if [ -e "${binary}" ]
    then
        mv "${binary}" "./${APP_NAME}.exe"
        7z a "${asset_binary_name}.zip" "./${APP_NAME}.exe"
        echo "asset_binary=${asset_binary_name}.zip" >> $GITHUB_ENV
    else
        echo "file no exists!!! binary :${binary}"
        exit 1
  fi

else
  binary="./target/${MATRIX_TARGET}/release/${APP_NAME}"
  if [ -e "${binary}" ]
  then
#      mkdir -p ./brew/bin
      mv "${binary}" "./${APP_NAME}"
#      cd ./brew || exit 1
      tar -czf "${asset_binary_name}.tar.gz"  "./${APP_NAME}"
#      cd ..
      echo "asset_binary=${asset_binary_name}.tar.gz" >> $GITHUB_ENV
  else
      echo "file no exists!!! binary :${binary}"
      exit 1
  fi
fi
echo "=================pack target binary================end===================="

echo "succeed."