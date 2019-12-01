package day1

import "core:fmt"
import "core:os"
import "core:strings"
import "core:strconv"

fuel_for_module :: proc(module_weight: int) -> int {
  return module_weight / 3 - 2;
}

main :: proc() {
  assert(fuel_for_module(12) == 2);
  assert(fuel_for_module(14) == 2);
  assert(fuel_for_module(1969) == 654);
  assert(fuel_for_module(100756) == 33583);

  data, success := os.read_entire_file(os.args[1]);

  if !success {
    fmt.eprintln("Could not read file.");
    return;
  }

  module_weights := strings.split(string(data), "\n");
  total_fuel_weight := 0;

  for module_weight in module_weights {
    if module_weight == "" { continue };
    total_fuel_weight += fuel_for_module(strconv.parse_int(module_weight));
  }

  fmt.printf("Total fuel weight: %v\n", total_fuel_weight);
}
