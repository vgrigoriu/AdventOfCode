from dataclasses import dataclass

from input import read_input, read_input_as_string

day = 5


@dataclass(frozen=True)
class RangeMapping:
    src_start: int
    dst_start: int
    length: int

    def src_end(self) -> int:
        return self.src_start + self.length - 1

    def __repr__(self):
        return f"[{self.src_start}, {self.src_end()}] -> [{self.dst_start}, {self.dst_start + self.length - 1}]"

    def __getitem__(self, item) -> int:
        if item not in self:
            raise IndexError(f"Index {item} is not in range {self}")
        return self.dst_start + (item - self.src_start)

    def __contains__(self, item) -> bool:
        return self.src_start <= item < self.src_start + self.length


@dataclass(frozen=True)
class Mapping:
    ranges: list[RangeMapping]

    def __getitem__(self, item) -> int:
        for range in self.ranges:
            if item in range:
                return range[item]
        return item


def build_mapping(mapping_lines: list[str]) -> Mapping:
    def range_mapping(line: str) -> RangeMapping:
        dst_range_start, src_range_start, length = line.split()
        return RangeMapping(int(src_range_start), int(dst_range_start), int(length))

    ranges = [range_mapping(line) for line in mapping_lines]
    return Mapping(sorted(ranges, key=lambda r: r.src_start))


def solve_part_1():
    puzzle_input = read_input_as_string(day, debug=True)
    paragraphs = [paragraph for paragraph in puzzle_input.split("\n\n") if paragraph.strip() != ""]

    seeds_paragraph = paragraphs[0]
    seeds = [int(seed) for seed in seeds_paragraph.split(": ")[1].split()]

    seed_to_soil_map = build_mapping(paragraphs[1].split("\n")[1:])
    soil_to_fertilizer_map = build_mapping(paragraphs[2].split("\n")[1:])
    fertilizer_to_water_map = build_mapping(paragraphs[3].split("\n")[1:])
    water_to_light_map = build_mapping(paragraphs[4].split("\n")[1:])
    light_to_temperature_map = build_mapping(paragraphs[5].split("\n")[1:])
    temperature_to_humidity_map = build_mapping(paragraphs[6].split("\n")[1:])
    humidity_to_location_map = build_mapping(paragraphs[7].split("\n")[1:])

    def final_location(seed: int) -> int:
        soil = seed_to_soil_map[seed]
        fertilizer = soil_to_fertilizer_map[soil]
        water = fertilizer_to_water_map[fertilizer]
        light = water_to_light_map[water]
        temperature = light_to_temperature_map[light]
        humidity = temperature_to_humidity_map[temperature]
        return humidity_to_location_map[humidity]

    final_locations = [final_location(seed) for seed in seeds]
    return min(final_locations)


@dataclass(frozen=True)
class Range:
    start: int
    length: int

    def __repr__(self):
        return f"[{self.start}, {self.end()}]"

    def end(self) -> int:
        return self.start + self.length - 1

    def apply_mapping(self, mapping: RangeMapping, debug: bool = False):
        if mapping.src_end() < self.start:
            print(f"Mapping {mapping} does not overlap with range {self} (before)") if debug else 1
            return [self]
        if self.end() < mapping.src_start:
            print(f"Mapping {mapping} does not overlap with range {self} (after)") if debug else 2
            return [self]
        result = []
        if self.start < mapping.src_start:
            result.append(Range(self.start, mapping.src_start - self.start))
        start = max(self.start, mapping.src_start)
        end = min(self.end(), mapping.src_end())
        result.append(Range(mapping[start], end - start + 1))
        if mapping.src_end() < self.end():
            result.append(Range(mapping.src_end() + 1,
                                self.end() - mapping.src_end()))
        print(f"Mapping {mapping} applied to range {self} gives {result}") if debug else 3
        return result


@dataclass(frozen=True)
class Ranges:
    ranges: list[Range]

    def apply_mapping(self, mapping: Mapping, debug: bool = False):
        result = self.ranges
        for mapping in mapping.ranges:
            result = [new_range
                      for old_range in result
                      for new_range in old_range.apply_mapping(mapping, debug)]
        return Ranges(sorted(result, key=lambda r: r.start))


def solve_part_2():
    puzzle_input = read_input_as_string(day, debug=True)
    paragraphs = [paragraph for paragraph in puzzle_input.split("\n\n") if paragraph.strip() != ""]

    seeds_paragraph = paragraphs[0]
    seed_range_definitions = [int(seed) for seed in seeds_paragraph.split(": ")[1].split()]
    seed_ranges = [Range(seed_range_definitions[i], seed_range_definitions[i + 1]) for i in
                   range(0, len(seed_range_definitions), 2)]
    seeds = Ranges(sorted(seed_ranges, key=lambda r: r.start))
    print(f"Seeds: {seeds}")

    seed_to_soil_map = build_mapping(paragraphs[1].split("\n")[1:])
    soil_to_fertilizer_map = build_mapping(paragraphs[2].split("\n")[1:])
    fertilizer_to_water_map = build_mapping(paragraphs[3].split("\n")[1:])
    water_to_light_map = build_mapping(paragraphs[4].split("\n")[1:])
    light_to_temperature_map = build_mapping(paragraphs[5].split("\n")[1:])
    temperature_to_humidity_map = build_mapping(paragraphs[6].split("\n")[1:])
    humidity_to_location_map = build_mapping(paragraphs[7].split("\n")[1:])

    print(f"Seed to soil: {seed_to_soil_map}")
    soils = seeds.apply_mapping(seed_to_soil_map)
    print(f"Soils: {soils}")
    print(f"Soil to fertilizer: {soil_to_fertilizer_map}")
    fertilizers = soils.apply_mapping(soil_to_fertilizer_map)
    print(f"Fertilizers: {fertilizers}")
    print(f"Fertilizer to water: {fertilizer_to_water_map}")
    waters = fertilizers.apply_mapping(fertilizer_to_water_map, debug=True)
    print(f"Waters: {waters}")
    lights = waters.apply_mapping(water_to_light_map)
    print(f"Lights: {lights}")
    temperatures = lights.apply_mapping(light_to_temperature_map)
    print(f"Temperatures: {temperatures}")
    humidities = temperatures.apply_mapping(temperature_to_humidity_map)
    print(f"Humidities: {humidities}")
    locations = humidities.apply_mapping(humidity_to_location_map)
    print(f"Locations: {locations}")

    return min(locations.ranges, key=lambda r: r.start).start


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
