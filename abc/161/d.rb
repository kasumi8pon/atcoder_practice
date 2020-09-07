k = gets.to_i

queue = (1..9).to_a
ponpon_numbers = []

until queue.empty? || ponpon_numbers.size == k
  current = queue.shift
  ponpon_numbers << queue.shift

  last_digit = current % 10
  followings = (last_digit - 1..last_digit + 1).reject { |num|
    num == -1 || num == 10
  }.map { |num| current * 10 + num }
  queue.push *followings
end

puts ponpon_numbers[k - 1]
