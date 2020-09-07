n = gets.to_i

require 'prime'

divisions = Prime.prime_division(n)

answer = 0

divisions.each do |prime, num|
  remain = num
  count = 1
  while remain >= count
    remain -= count
    answer += 1
    count += 1
  end
end

puts answer
