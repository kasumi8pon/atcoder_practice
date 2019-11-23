a, b = gets.split.map(&:to_i)

answer = -1
answer = a * b if (a >= 1 && a <= 9) && (b >= 1 && b <= 9)

puts answer
