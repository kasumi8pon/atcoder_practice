n = gets.to_i
s = gets.split.map(&:to_i)

answer = 0

d.combination(2).each { |x, y| answer += x * y }

puts answer
