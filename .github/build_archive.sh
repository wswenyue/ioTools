#!/usr/bin/env bash
# Author: wswenyue.
#Date & Time: 2024-03-15 19:55:32
#Description: a bash script
binary_name="${APP_NAME}"
version="${APP_VERSION}"
# shellcheck disable=SC2034
dir_name="${binary_name}-${version}-${MATRIX_TARGET}"
mkdir "${dir_name}"
# shellcheck disable=SC2193
if [ "${{ MATRIX_OS }}" = "windows-latest" ]; then
  mv "target/${MATRIX_TARGET}/release/${binary_name}.exe" "${dir_name}"
else
  mv "target/${MATRIX_TARGET}/release/${binary_name}" "${dir_name}"
fi

# shellcheck disable=SC2193
if [ "${{ MATRIX_OS }}" = "windows-latest" ]; then
  7z a "${dir_name}.zip" "${dir_name}"
  echo "ASSET=${dir_name}.zip" >> $GITHUB_ENV
else
  tar -czf "${dir_name}.tar.gz" "${dir_name}"
  echo "ASSET=${dir_name}.tar.gz" >> $GITHUB_ENV
fi