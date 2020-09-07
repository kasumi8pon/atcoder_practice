n = gets.to_i
friendlies = gets.split.map(&:to_i)

comfortables = (friendlies * 2).sort!.reverse!

puts comfortables[1...n].sum
