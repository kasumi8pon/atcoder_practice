l, r = gets.split.map(&:to_i)

range = [2019, r - l + 1].min

mods = range.times.map { |i| (l + i) % 2019 }

puts mods.combination(2).map { |a, b| (a * b) % 2019 }.min
