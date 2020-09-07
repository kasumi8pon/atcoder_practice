card = []
3.times do
  card << gets.split.map(&:to_i)
end
n = gets.to_i
numbers = []
n.times do
  numbers << gets.to_i
end

card.flatten!
finished = card.map { |num| numbers.include?(num) ? 0 : num }

def bingo?(c)
  return false if c.select {|n| n == 0 }.size < 3

  c[0] == 0 && c[1] == 0 && c[2] == 0 \
    || c[3] == 0 && c[4] == 0 && c[5] == 0 \
    || c[6] == 0 && c[7] == 0 && c[8] == 0 \
    || c[0] == 0 && c[3] == 0 && c[6] == 0 \
    || c[1] == 0 && c[4] == 0 && c[7] == 0 \
    || c[2] == 0 && c[5] == 0 && c[8] == 0 \
    || c[0] == 0 && c[4] == 0 && c[8] == 0 \
    || c[2] == 0 && c[4] == 0 && c[6] == 0 \
end

puts bingo?(finished) ? 'Yes' : 'No'
