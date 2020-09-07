number = gets.to_i
heights = gets.split.map(&:to_i)

answer = 0

until heights.all? { |height| height.zero? } do
  min = nil
  max = nil
  0.upto(number - 1) do |i|
    next if heights[i].zero? && min.nil?

    min = i if min.nil?

    if min && heights[i].zero?
      answer += 1
      min = nil
      break
    end

    heights[i] -= 1

    answer += 1 if i == (number - 1)
  end
end

puts answer
