a, b = gets.split.map(&:to_i)

answer_x = nil
answer_y = nil

(-100..100).each do |x|
  y = a - x
  if x - y == b
    answer_x = x
    answer_y = y
  end
end

puts "#{answer_x} #{answer_y}"
