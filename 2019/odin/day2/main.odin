package day2

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

run_program :: proc(program: []int) -> []int {
  loop: for i := 0; i < len(program); i += 1 {
    token := program[i];

    switch token {
      case 1:
        x := program[i + 1];
        y := program[i + 2];
        dest := program[i + 3];

        program[dest] = program[x] + program[y];
        i += 3;

      case 2:
        x := program[i + 1];
        y := program[i + 2];
        dest := program[i + 3];

        program[dest] = program[x] * program[y];
        i += 3;

      case 99:
        break loop;
    }
  }

  return program;
}

main :: proc() {
  data, success := os.read_entire_file(os.args[1]);

  if !success {
    fmt.eprintln("Could not read file.");
    return;
  }

  stringified := strings.split(string(data), ",");
  intcode := make([]int, len(stringified));

  for x, i in stringified {
    intcode[i] = strconv.parse_int(x);
  }

  intcode[1] = 12;
  intcode[2] = 2;

  out := run_program(intcode[:]);
  fmt.printf("Program position 0: %v\n", out[0]);
}
