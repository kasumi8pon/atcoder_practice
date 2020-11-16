numbers = gets.chomp.split('')
tally = numbers.tally
tally.default = 0

last_numbers = (0..124).map { |num| num * 8 }

answer = 'No'

if numbers.size > 2
  last_numbers.each do |num|
    snum = num < 100 ? num.to_s.rjust(3, '0') : num.to_s
    snum_tally = snum.split('').tally
    if snum_tally.all? { |k, v| tally[k] >= v }
      answer = 'Yes'
      break
    end
  end
elsif numbers.size == 2
  answer = 'Yes' if "#{numbers.first}#{numbers.last}".to_i % 8 == 0 || "#{numbers.last}#{numbers.first}".to_i % 8 == 0
else
  answer = 'Yes' if numbers.join.to_i % 8 == 0
end

puts answer
