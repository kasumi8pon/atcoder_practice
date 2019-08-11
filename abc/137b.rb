k, x = gets.chomp.split(" ").map(&:to_i)

answer = Array.new(k * 2 - 1) { |i| x - k + 1 + i }

puts answer.join(" ")
