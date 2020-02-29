n = gets.to_i
a = gets.split.map(&:to_i)

numbers = a

answer = 'APPROVED'

numbers.each do |num|
  next unless num.even?

  unless num % 3 == 0 || num % 5 == 0
    answer = 'DENIED'
    break
  end
end

puts answer
