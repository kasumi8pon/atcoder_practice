n = gets.to_i
stones = gets.chomp.split('')

answer = 0

change_r = 0
change_w = 0

if stones.any? { |stone| stone == 'W' }
  red = stones.count { |stone| stone == 'R' }
  stones.each.with_index(1) do |stone, i|
    if i <= red
      change_r += 1 if stone == 'W'
    else
      change_w += 1 if stone == 'R'
    end
  end

  answer = change_r + change_w - [change_r, change_w].min
end

puts answer
