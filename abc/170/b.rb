x, y = gets.split.map(&:to_i)

answer = 'No'

(0..x).each do |tsuru_num|
  answer = 'Yes' if 4 * tsuru_num + 2 * (x - tsuru_num) == y
end

puts answer
