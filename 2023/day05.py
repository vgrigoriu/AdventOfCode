from dataclasses import dataclass

from input import read_input, read_input_as_string

day = 5


@dataclass(frozen=True)
class RangeMapping:
    src_range_start: int
    dst_range_start: int
    length: int

    def __getitem__(self, item) -> int:
        if item not in self:
            raise IndexError(f"Index {item} is not in range {self}")
        return self.dst_range_start + (item - self.src_range_start)

    def __contains__(self, item) -> bool:
        return self.src_range_start <= item < self.src_range_start + self.length


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
    return Mapping(ranges)


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

    def apply_mapping(self, mapping: RangeMapping):
        if mapping.src_range_start + mapping.length <= self.start:
            return [self]
        if self.start + self.length <= mapping.src_range_start:
            return [self]
        result = []
        if self.start < mapping.src_range_start:
            result.append(Range(self.start, mapping.src_range_start - self.start))
        start = max(self.start, mapping.src_range_start)
        end = min(self.start + self.length, mapping.src_range_start + mapping.length)
        result.append(Range(mapping[start], end - start))
        if mapping.src_range_start + mapping.length < self.start + self.length:
            result.append(Range(mapping.dst_range_start + mapping.length,
                                self.start + self.length - mapping.src_range_start - mapping.length))
        return result


@dataclass(frozen=True)
class Ranges:
    ranges: list[Range]

    def apply_mapping(self, mapping: Mapping):
        result = self.ranges
        for mapping in mapping.ranges:
            result = [new_range
                      for old_range in result
                      for new_range in old_range.apply_mapping(mapping)]
        return Ranges(sorted(result, key=lambda r: r.start))


def solve_part_2():
    puzzle_input = read_input_as_string(day, debug=True)
    paragraphs = [paragraph for paragraph in puzzle_input.split("\n\n") if paragraph.strip() != ""]

    seeds_paragraph = paragraphs[0]
    seed_range_definitions = [int(seed) for seed in seeds_paragraph.split(": ")[1].split()]
    seed_ranges = [Range(seed_range_definitions[i], seed_range_definitions[i + 1]) for i in
                   range(0, len(seed_range_definitions), 2)]
    seeds = Ranges(seed_ranges)
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
    fertilizers = soils.apply_mapping(soil_to_fertilizer_map)
    print(f"Fertilizers: {fertilizers}")
    waters = fertilizers.apply_mapping(fertilizer_to_water_map)
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
