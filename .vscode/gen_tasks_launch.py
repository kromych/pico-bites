#!/usr/bin/env python3

import json
import glob

from string import Template

def substitute(obj, **kwd):
    if isinstance(obj, dict):
        new_dict = dict()
        for key, value in obj.items():
            new_dict[key] = substitute(value, **kwd)
        return new_dict
    elif isinstance(obj, list):
        new_list = list()
        for item in obj:
            new_list.append(substitute(item, **kwd))
        return new_list
    elif isinstance(obj, set):
        new_set = set()
        for item in obj:
            new_set.add(substitute(item, **kwd))
        return new_set
    elif isinstance(obj, str):   
        template = Template(obj)
        return template.safe_substitute(**kwd)     
    else: 
        return obj

task_cargo_example_template = {
    "label": "cargo build --example ${example_name}",
    "type": "shell",
    "command": "cargo",
    "args": [
        "build",
        "--example",
        "${example_name}"
    ],
    "problemMatcher": [
        "$rustc"
    ],
    "group": "build"
}

task_binary_example_template = {
    "label": "build example ${example_name}",
    "type": "shell",
    "command": "arm-none-eabi-objcopy",
    "args": [
        "--output-target",
        "binary",
        "./target/thumbv6m-none-eabi/debug/examples/${example_name}",
        "./target/thumbv6m-none-eabi/debug/examples/${example_name}.bin"
    ],
    "problemMatcher": [
        "$rustc"
    ],
    "group": {
        "kind": "build",
        "isDefault": True
    },
    "dependsOn": "cargo build --example ${example_name}"
}

probe_rs_debug_launch_example_template = {
    "preLaunchTask": "cargo build --example ${example_name}",
    "type": "probe-rs-debug",
    "request": "launch",
    "name": "probe-rs-debug ${example_name}",
    "cwd": "$${workspaceFolder}",
    "chip": "rp2040",
    # RP2040 doesn't support connectUnderReset
    "connectUnderReset": False,
    "speed": 4000,
    "runtimeExecutable": "probe-rs-debugger",
    "runtimeArgs": [
        "debug"
    ],
    "flashingConfig": {
        "flashingEnabled": True,
        "resetAfterFlashing": True,
        "haltAfterReset": True,
    },
    "coreConfigs": [
        {
            "coreIndex": 0,
            "programBinary": "target/thumbv6m-none-eabi/debug/examples/${example_name}",
            "chip": "RP2040",
            # https://github.com/raspberrypi/pico-sdk/raw/master/src/rp2040/hardware_regs/rp2040.svd
            "svdFile": "$${workspaceFolder}/.vscode/rp2040.svd",
            "rttEnabled": True,
            "options": {
                "env": {
                    "DEFMT_LOG": "debug"
                }
            },
        }
    ],
    "consoleLogLevel": "Info", # Error, Warn, Info, Debug, Trace
    "wireProtocol": "Swd"
}

cortex_debug_cmsis_dap_launch_example_template = {
    "preLaunchTask": "cargo build --example ${example_name}",
    "name": "Cortex Debug CMSIS-DAP ${example_name}",
    "cwd": "$${workspaceRoot}",
    # "executable": "$${command:cmake.launchTargetPath}",
    "executable": "$${workspaceFolder}/target/thumbv6m-none-eabi/debug/examples/${example_name}",
    "request": "launch",
    "type": "cortex-debug",
    "servertype": "openocd",
    # "gdbPath": "gdb-multiarch",
    "gdbPath": "arm-none-eabi-gdb",
    "device": "RP2040",
    "configFiles": [
        "interface/cmsis-dap.cfg",
        "target/rp2040.cfg"
    ],
    "svdFile": "$${workspaceRoot}/.vscode/rp2040.svd",
    "runToEntryPoint": "main",
    "preLaunchCommands": [
        "monitor init",
        "monitor reset init",
        "monitor halt",
        "monitor arm semihosting enable",
        "monitor arm semihosting_fileio enable",
    ],
    # Workaround for stopping at main on restart
    "postRestartCommands": [
        "break main",
        "continue"
    ],
    "openOCDLaunchCommands": [
        "adapter speed 5000",
    ]
}

external_openocd_lauch_example_template =  {
    "preLaunchTask": "cargo build --example ${example_name}",
    "name": "External OpenOCD ${example_name}",
    "request": "launch",
    "type": "cortex-debug",
    "cwd": "$${workspaceRoot}",
    "executable": "$${workspaceFolder}/target/thumbv6m-none-eabi/debug/examples/${example_name}",
    "preLaunchTask": "build example ${example_name}",
    "servertype": "external",
    "gdbPath": "arm-none-eabi-gdb",
    # Connect to an already running OpenOCD instance
    # openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -s tcl -c "adapter speed 5000"
    "gdbTarget": "localhost:3333",
    "svdFile": "$${workspaceRoot}/.vscode/rp2040.svd",
    "runToEntryPoint": "main",
    "preLaunchCommands": [
        "monitor init",
        "monitor reset init",
        "monitor halt",
        "monitor arm semihosting enable",
        # "monitor arm semihosting_fileio enable",
        "target extended-remote :3333",
        "set print asm-demangle on",
        "set backtrace limit 32",
        # "break DefaultHandler",
        # "break HardFault",
        "break main",
        # "load",
        # "stepi",
    ],
    # Work around for stopping at main on restart
    "postRestartCommands": [
        "break DefaultHandler",
        "break HardFault",
        "break main",
        "continue"
    ],
}

# The format of this file is specified in https://probe.rs/docs/tools/vscode/#start-a-debug-session-with-minimum-configuration
launch_configs = {
    "version": "0.2.0",
    "configurations": []
}

# See https://go.microsoft.com/fwlink/?LinkId=733558 
# for the documentation about the tasks.json format
tasks = {
    "version": "2.0.0",
    "tasks": []
}

for example in glob.glob("*.rs", root_dir="examples"):
    example_name = example[:-3]

    for task in [task_binary_example_template, task_cargo_example_template]:
        subs = substitute(task, example_name=example_name)
        tasks["tasks"].append(subs)

    for config in [probe_rs_debug_launch_example_template,
                   cortex_debug_cmsis_dap_launch_example_template,
                   external_openocd_lauch_example_template]:
        subs = substitute(config, example_name=example_name)
        launch_configs["configurations"].append(subs)

with open("tasks.json", "wt") as tasks_json:
    tasks_json.write(json.dumps(obj=tasks, indent=4))

with open("launch.json", "wt") as launch_json:
    launch_json.write(json.dumps(obj=launch_configs, indent=4))
