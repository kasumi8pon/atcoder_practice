n = gets.to_i
a_numbers = gets.split.map(&:to_i)

multiples_of_four = a_numbers.count { |number| number % 4 == 0 }
multiples_of_two_not_four = a_numbers.count(&:even?) - multiples_of_four
rest = n - (multiples_of_four + multiples_of_two_not_four)

if multiples_of_two_not_four.zero?
  puts multiples_of_four + 1 >= rest ? 'Yes' : 'No'
else
  puts multiples_of_four + 1 >= rest + 1 ? 'Yes' : 'No'
end
