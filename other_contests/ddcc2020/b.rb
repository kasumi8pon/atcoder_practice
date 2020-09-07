n = gets.to_i
a = gets.split.map(&:to_i)

full_length = a.inject(:+)
half_length = full_length / 2

left = 0
half_index = 0
a.each_with_index do |length, i|
  left += length
  if left >= half_length
    half_index = i
    break
  end
end

right = full_length - left

answers = []
answers << (right - left).abs
answers << (right + a[half_index] - (left - a[half_index])).abs

puts answers.min
