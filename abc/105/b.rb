n = gets.chomp.to_i

max = n / 7
result = "No"

(0..max).each do |i|
  if (n - 7 * i) % 4 == 0
    result = "Yes"
    break
  end
end

puts result