from parseopt import getopt

type
  ArgType = enum
    Glob
    Command

  Argument = object
    kind: ArgType
    value: string

proc interpret_arg*(flag, arg: string): Argument =
  case flag
  of "w":
    echo Argument(kind: Glob, value: arg)
  of "x":
    echo Argument(kind: Command, value: arg)
    return

proc parse_cli_params*(args: seq[string]) =
  return
