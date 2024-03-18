#!/usr/bin/env python
# -*- coding: utf-8 -*-
# Created by wswenyue on 2024-03-17 20:51:42.
# Description: a python script
import json
import os
import subprocess
import uuid
from typing import Dict, Any


def get_str(obj) -> str:
    if type(obj) is str:
        return obj
    elif type(obj) is bytes:
        return bytes.decode(obj, encoding="utf-8", errors="ignore")
        # return obj.decode("utf-8")
    else:
        return str(obj)


def add_envs(envs: Dict[str, Any]):
    env_file = os.getenv('GITHUB_ENV')
    # write to the file
    with open(env_file, "a") as env_file:
        for key, value in envs.items():
            env_file.write(f"{key}={value}")
            env_file.write("\n")


def set_output(name, value):
    with open(os.environ['GITHUB_OUTPUT'], 'a') as fh:
        print(f'{name}={value}', file=fh)


def set_multiline_output(name, value):
    with open(os.environ['GITHUB_OUTPUT'], 'a') as fh:
        delimiter = uuid.uuid1()
        print(f'{name}<<{delimiter}', file=fh)
        print(value, file=fh)
        print(delimiter, file=fh)


def cmd_run_shell(cmd: str) -> str:
    ps = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    output = ps.communicate()[0]
    return get_str(output)


def cmd_run(cmd: str) -> str:
    _cmd = str(cmd).split()
    output = subprocess.check_output(_cmd)
    return get_str(output)


def cmd_run_iter(cmd: str):
    _cmd = str(cmd).split()
    _popen = subprocess.Popen(_cmd, stdout=subprocess.PIPE, universal_newlines=True)
    for stdout_line in iter(_popen.stdout.readline, ""):
        yield get_str(stdout_line)
    _popen.stdout.close()
    return_code = _popen.wait()
    if return_code:
        raise subprocess.CalledProcessError(return_code, _cmd)


def is_empty(obj):
    if obj is None:
        return True
    if not obj:
        return True
    if type(obj) is str:
        if obj == "":
            return True
        elif obj.strip() == "":
            return True
        else:
            return False
    if type(obj) is list:
        if len(obj) <= 0:
            return True
        else:
            return False
    if type(obj) is dict:
        return bool(obj)
    else:
        return False


def is_not_empty(obj):
    return not is_empty(obj)


def next_revision_num(version_pre: str) -> int:
    _max = -1
    for line in cmd_run_iter("git ls-remote --tags -q"):
        if is_empty(line):
            continue
        if version_pre not in line:
            continue
        code = int(line.rsplit('.', 1)[-1])
        # print(f"code:{code}")
        if _max < code:
            _max = code
    return _max + 1


def cargo_meta_handle() -> Dict:
    meta: str = cmd_run("cargo metadata --no-deps --format-version 1")
    meta_json = json.loads(meta)
    app_version = meta_json['packages'][0]['version']
    print(f"app_version:{app_version}")
    app_name = meta_json['packages'][0]['name']
    maintainer = meta_json['packages'][0]['authors'][0]
    homepage = meta_json['packages'][0]['homepage']
    rust_version = meta_json['packages'][0]['rust_version']
    publish_info = meta_json['packages'][0]['metadata']['info']['publish_info']
    return {
        "APP_VERSION": app_version,
        "APP_NAME": app_name,
        "MAINTAINER": maintainer,
        "HOMEPAGE": homepage,
        "RUST_VERSION": rust_version,
        "PUBLISH_INFO": publish_info,
    }


def run(cfg: Dict[str, str]):
    version = cfg['APP_VERSION']
    print(f"version: {version}")
    v_revision = next_revision_num(version)
    new_version = f"{version}.{v_revision}"
    new_tag = f"v{new_version}"
    print(f"new_tag: {new_tag}")
    cfg['NEW_VERSION'] = new_version
    cfg['NEW_TAG'] = new_tag
    add_envs(cfg)


if __name__ == '__main__':
    run(cargo_meta_handle())
