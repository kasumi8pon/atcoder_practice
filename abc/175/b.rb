n = gets.to_i
sticks = gets.split.map(&:to_i)

answer = 0

sticks.combination(3).each do |a, b, c|
  next if [a, b, c].uniq.size != 3

  answer += 1 if (a + b) > c && (b + c) > a && (c + a) > b
end

puts answer
