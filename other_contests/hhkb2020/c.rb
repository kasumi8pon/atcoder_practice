n = gets.to_i
numbers = gets.split.map(&:to_i)

rest_min = 0
allow_numbers = Array.new(200001) { 1 }

answers = []

numbers.each do |num|
  allow_numbers[num] = 0

  while allow_numbers[rest_min] == 0
    rest_min += 1
  end

  answers << rest_min
end

puts answers
