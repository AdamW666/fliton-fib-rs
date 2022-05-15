import argparse
import yaml
import os
from pprint import pprint
from .fliton_fib_rs import run_config


def config_number_command() -> None:
    parser = argparse.ArgumentParser(
        description="Calculate Fibonacci from config file")
    parser.add_argument('--path', action='store', type=str,
                        required=True, help="Path of the config file")
    args = parser.parse_args()

    with open(str(os.getcwd()) + "/" + args.path) as fh:
        config_data: dict = yaml.safe_load(fh)

    print("Here is the config data: ")
    pprint(config_data)
    print("Here is the result: ")
    pprint(run_config(config_data))
