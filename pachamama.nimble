# Package

from os import walk_dir_rec
from strutils import contains

version       = "0.1.0"
author        = "Anthony"
description   = "Filewatcher and task runner"
license       = "MIT"
srcDir        = "src"
bin           = @["pacha"]

task test, "Run tests":
  for file in walk_dir_rec "src/":
    if contains(file, "test.nim"):
      exec "nim c -r -o:/tmp $#" % file

task watch, "Watch files for changes":
  exec "find -name *.nim | entr -c --hint:off nimble test"

# Dependencies

requires "nim >= 1.0.4"
