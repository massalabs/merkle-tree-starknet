import os
import argparse
import time
import traceback
from icecream import ic

import asyncio
import random
from typing import Callable, Iterable, List, Set, Tuple

from queue import Queue

import sys

from .test_tree import ITestTree


from .command_interpreter import CommandInterpreter
from .bindings import (
    CommandList,
    free_scenario,
    load_random,
    load_scenario,
)


async def run_test(
    test_env_creator: Callable[[], ITestTree],
    test_path: str,
    fix: bool = False,
    rand: bool = False,
    debug: bool = False,
):
    fix_path: str | None = None
    match (rand, fix):
        case (True, _):  # random_mode => fix_mode
            fix_path = (
                test_path
                + "random_"
                + str(int(time.time()))
                + "_"
                + str(random.randint(0, 10000))
                + ".yml"
            )

        case (False, True):
            fix_path = test_path.replace(".yml", "_fix.yml")

        case (False, False):
            fix_path = None

    interpreter = await CommandInterpreter.create(
        test_env_creator, fix_path, debug=debug
    )

    commands: CommandList = load_scenario(test_path) if not rand else load_random()

    vec_cmd = [commands.test_commands[i] for i in range(commands.len)]

    for cmd in vec_cmd:
        await interpreter.run_command(cmd)

    free_scenario(commands)


async def init_test(test_env_creator: Callable[[], ITestTree]):
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("-f", "--scenario_file", nargs=1, help="scenario to run")
    group.add_argument(
        "-d",
        "--scenario_dir",
        nargs=1,
        type=str,
        help="directory to find scenarios to run",
    )
    group.add_argument(
        "-r",
        "--random",
        nargs=2,
        metavar=("PATH", "NUMBER"),
        help="generate NUMBER random scenarios into PATH directory",
    )
    parser.add_argument(
        "--fix",
        default=False,
        action="store_true",
        help="write a copy of the scenario with the expected output filled in",
    )
    parser.add_argument(
        "-D", "--debug", action="store_true", help="enable debug output"
    )

    args = parser.parse_args()

    tests: list[str]
    if args.scenario_file:
        tests = [args.scenario_file[0]]
    elif args.scenario_dir:
        tests = [
            args.scenario_dir[0] + file
            for file in os.listdir(args.scenario_dir[0])  # type: ignore
            if file.endswith(".yml")  # type: ignore
        ]
        tests.sort()
    elif args.random:
        dir_path, num = args.random
        tests = [dir_path for _ in range(int(num))]
    else:
        raise ValueError("No scenario or scenario_dir provided")

    for test in tests:
        print(f"Running test {test}")
        try:
            await run_test(
                test_env_creator=test_env_creator,
                test_path=test,
                fix=args.fix,
                rand=args.random is not None,
                debug=args.debug,
            )

            print(f"Test {test} SUCCESS")
            if args.fix and not args.random:
                print("Test fixed as", test.replace(".yml", "_fix.yml"))
            if args.random:
                print(f"New random generated test written")
        except Exception as e:
            print(f"Test {test} FAIL with: ", e)
            print(traceback.format_exc())
            continue
