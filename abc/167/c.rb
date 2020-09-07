n, m, x = gets.split.map(&:to_i)
textbooks = []
n.times do
  input = gets.split.map(&:to_i)
  textbooks << { price: input[0], progress: input[1..-1] }
end

total_prices = []

(2 ** n).times do |i|
  skills = Array.new(m) { 0 }
  price = 0

  textbooks.each_with_index do |textbook, bit_i|
    next if i[bit_i].zero?

    textbook[:progress].each_with_index do |progress, progress_i|
      skills[progress_i] += progress
    end

    price += textbook[:price]
  end

  next if skills.any? { |skill| skill < x }

  total_prices << price
end

puts total_prices.min || -1
