require 'complex'

n = gets.to_i
root_n = Math.sqrt(n).floor

answers = []

root_n.downto(1) do |a|
  next if n % a != 0

  b = n / a
  answers << a + b - 2
end

puts answers.min
