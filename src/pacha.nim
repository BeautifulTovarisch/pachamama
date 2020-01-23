from os import param_count, command_line_params

from watch/watch import parse_cli_params

proc main() =

  parse_cli_params(command_line_params())

main()
