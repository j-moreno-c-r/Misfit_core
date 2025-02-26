from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser, Namespace
from importlib import import_module
import sys
import os
from typing import List, Optional

COMMANDS_DIR = "commands"

class MisfitCore:
    def __init__(self, args: Optional[List[str]] = None) -> str:
        parsed = self.parse_args(args)

        if not parsed.command:
            self.parser.print_help()
            sys.exit("No command provided.")

        module_path = f"src.{COMMANDS_DIR}.{parsed.command}"
        module = import_module(module_path)
        
        if hasattr(module, 'execute'):
            return module.execute(parsed)

    def get_commands(self) -> list[str]:
        commands = []

        if not os.path.exists(COMMANDS_DIR):
            return commands
        
        # Get all python files inside COMMANDS_DIR
        for filename in os.listdir(f"src/{COMMANDS_DIR}"):
            if filename.endswith(".py") and not filename.startswith("__"):
                commands.append(filename[:-3])

        return commands

    def parse_args(self, args: Optional[list[str]] = None) -> Namespace:
        commands = self.get_commands()

        self.parser = ArgumentParser(
            description="A tool for create specified invalid parameters for tests in bicoin.",
            formatter_class=ArgumentDefaultsHelpFormatter
        )
        
        self.parser.add_argument('--verbose', action='store_true', help='Enable verbose output')
        # TODO: parser.add_argument('--config', default='config.ini', help='Configuration file')
        
        subparsers = self.parser.add_subparsers(title="Commands", dest="command", help="Available commands")

        for pyfile in commands:
            module_path = f"src.{COMMANDS_DIR}.{pyfile}"
            module = import_module(module_path)
            module.register_parser(subparsers)
        
        return self.parser.parse_args(args)

def main():
    os.makedirs(COMMANDS_DIR, exist_ok=True)
    sys.exit(MisfitCore())

if __name__ == "__main__":
    main()
