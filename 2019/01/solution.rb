def fuel_for_mass(mass)
    mass / 3 - 2
end

def total_fuel_for_mass(mass)
    fuel = fuel_for_mass(mass)
    if fuel <= 0
        0
    else
        fuel + total_fuel_for_mass(fuel)
    end
end

puts fuel_for_mass(12)
puts total_fuel_for_mass(12)
puts fuel_for_mass(14)
puts total_fuel_for_mass(14)
puts fuel_for_mass(1969)
puts total_fuel_for_mass(1969)
puts fuel_for_mass(100756)
puts total_fuel_for_mass(100756)

file = File.open("input")
masses = file.readlines.map(&:chomp).map(&:to_i)

puts masses

fuels = masses.map { |mass| fuel_for_mass(mass) }
total_fuels = masses.map { |mass| total_fuel_for_mass(mass) }

puts fuels

puts fuels.sum
puts total_fuels.sum
