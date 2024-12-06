#!/usr/bin/env python3

"""Debug Config Gen

Automatically generates CodeLLDB debug configurations for failing commands.

Expects at least one command line argument, which should be the absolute path to
the binary to execute.

Meant to be set as the `RUSTC_WRAPPER_REAL` environment variable.

Generates the debug configuration in `$DCG_LAUNCH_JSON` if specified, or in
`$PWD/.vscode/launch.json` (if the `.vscode` directory exists).
"""

import os
import sys
import json
import subprocess
import pathlib

def main():

    result = subprocess.run(sys.argv[1:], close_fds=False)

    if result.returncode == 0:
        return

    binary = pathlib.Path(sys.argv[1]).name or sys.argv[1]

    print("\n\n")
    print(f"[DCG] `{binary}` failed, attempting to generate debug config...")

    launch_path = os.environ.get('DCG_LAUNCH_JSON')
    if launch_path:
        print(f"[DCG] Using launch config from `$DCG_LAUNCH_JSON`: `{launch_path}`")
        launch_path = pathlib.Path(launch_path)
    else:
        vscode = pathlib.Path(os.getcwd(), ".vscode")
        if not vscode.is_dir():
            print(f"[DCG] Cannot determine launch config path, not a directory: `{vscode}`")
            print(f"[DCG] You can specify a launch config explicit in `$DCG_LAUNCH_JSON`.")
            print("\n\n")
            exit(result.returncode)
        launch_path = vscode.joinpath("launch.json")
        print(f"[DCG] Using launch config: `{launch_path}`")

    launch_path.parent.mkdir(parents=True, exist_ok=True)

    launch_json = {
            "version": "0.2.0",
            "configurations": [],
        }

    if launch_path.exists():
        with open(launch_path, "r") as f:
            launch_json = json.load(f)

    config_name = f'DCG-{binary}'

    config = {
        "type": "lldb",
        "request": "launch",
        "name": config_name,
        'program': sys.argv[1],
        'args': [arg for arg in sys.argv[2:] if not arg.startswith("--error-format")],
        'cwd': os.getcwd(),
        'env': dict(os.environ),
        'terminal': 'console',
    }

    for c in launch_json['configurations']:
        if c['name'] == config_name:
            c.clear()
            c.update(config)
            config = {}

    if config:
        launch_json['configurations'].append(config)


    with open(launch_path, 'w') as f:
        json.dump(launch_json, f, indent=4)

    print(f"[DCG] Generated `{config_name}` debug config.")
    print("\n\n")

    exit(result.returncode)


if __name__ == '__main__':
    main()
