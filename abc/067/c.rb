n = gets.to_i
numbers = gets.split.map(&:to_i)

sum = numbers.sum

snuke_sum = 0
abss = []

(0...n - 1).each do |i|
  snuke_sum += numbers[i]
  sum -= numbers[i]
  abss << (snuke_sum - sum).abs
end

puts abss.min
