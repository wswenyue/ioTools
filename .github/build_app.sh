#!/usr/bin/env bash
# Author: wswenyue.
#Date & Time: 2024-03-15 19:55:32
#Description: a bash script
ls -alh ./
echo "=============gui:${GUI_NAME}======build begin================"
cargo tauri build --verbose --target "${MATRIX_TARGET}"
echo "=============gui:${GUI_NAME}======build end================"
ls -alh ./

echo "----------------./target/release/-----------------------------"
ls -alh ./target/release/
echo "----------------./target/${MATRIX_TARGET}---------------------"
ls -alh "./target/${MATRIX_TARGET}"
echo "----------------./target/${MATRIX_TARGET}/release-------------"
ls -alh "./target/${MATRIX_TARGET}/release"
echo "--------------------------------------------------------------"

echo "=================pack app ${MATRIX_TARGET}=====begin============="
if [ "${MATRIX_OS}" = "macos-latest" ]; then
  #./target/x86_64-apple-darwin/release/bundle/dmg/io_tools_desk_1.1.0_x64.dmg
  if [ "${MATRIX_TARGET}" = "x86_64-apple-darwin" ]; then
    app_dmg="${GUI_NAME}_${APP_VERSION}_x64.dmg"
  else
    app_dmg="${GUI_NAME}_${APP_VERSION}_x32.dmg"
  fi
  app_path="./target/${MATRIX_TARGET}/release/bundle/dmg/${app_dmg}"

  if [ -e "${app_path}" ]; then
    mv "${app_path}" "./${app_dmg}"
    echo "ASSET_GUI=${app_dmg}" >>$GITHUB_ENV
  else
    echo "file no exists!!! app_dmg :${app_dmg}"
    exit 1
  fi
else
  echo "unSupport!!!"
fi
echo "=================pack app ================end===================="

echo "succeed."