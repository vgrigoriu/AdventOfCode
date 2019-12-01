def fuel_for_mass(mass)
    mass / 3 - 2
end

puts fuel_for_mass(12)
puts fuel_for_mass(14)
puts fuel_for_mass(1969)
puts fuel_for_mass(100756)

file = File.open("input")
masses = file.readlines.map(&:chomp).map(&:to_i)

puts masses

fuels = masses.map { |mass| fuel_for_mass(mass) }

puts fuels

puts fuels.sum