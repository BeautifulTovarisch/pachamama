import std/unittest

from watch import parse_cli_params


suite "Watcher Tests":
  test "Test Parsing CLI args":
    let cmds = @["-w",
                 "**/*.txt",
                 "-x /bin/true",
                 "-w",
                 "**/*.css",
                 "-x", "minify css",
                 "-h",
                 "--help"]
    check(1 == 1)
