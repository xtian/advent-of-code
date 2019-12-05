package day2

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

run_program :: proc(program: []int) -> []int {
  loop: for i := 0; i < len(program); i += 1 {
    switch program[i] {
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

  for x in 0..99 {
    for y in 0..99 {
      intcode_copy := make([]int, len(intcode));
      defer delete(intcode_copy);

      copy(intcode_copy, intcode);

      intcode_copy[1] = x;
      intcode_copy[2] = y;

      out := run_program(intcode_copy[:]);

      if out[0] == 19690720 {
        fmt.printf("Program arg 1: %v\n", x);
        fmt.printf("Program arg 2: %v\n", y);

        return;
      }
    }
  }
}
