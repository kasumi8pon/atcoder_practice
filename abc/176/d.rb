$h, $w = gets.split.map(&:to_i)
start = gets.split.map(&:to_i)
$goal = gets.split.map(&:to_i)
grid = []
$h.times do
  grid << gets.chomp.split('')
end

pp grid

def neighborhoods(y, x)
  [[y - 1, x], [y, x - 1], [y, x + 1], [y + 1, x]]
end

def next_with_magix(y, x)
  [
    [y - 2, x - 2], [y - 2, x - 1], [y - 2, x], [y - 2, x + 1], [y - 2, x + 2],
    [y - 1, x - 2], [y - 1, x - 1], [y - 1, x + 1], [y - 1, x + 2],
    [y, x - 2], [y, x + 2],
    [y + 1, x - 2], [y + 1, x - 1], [y + 1, x + 1], [y + 1, x + 2],
    [y + 2, x - 2], [y + 2, x - 1], [y + 2, x], [y + 2, x + 1], [y + 2, x + 2]
]
end

$count = 0
$answers = []
$arrived = []

def recursive_step(y, x)
  if (y >= $h) || (y < 0) || (x >= $w) || (x < 0)
    $count = 0
    return
  end

  if (y == $goal.first - 1) && (x == $goal.last - 1)
    $answers << $count
    $count = 0
    return
  end

  neighborhoods(y, x).each do |next_y, next_x|
    recursive_step(next_y, next_x)
  end

  next_with_magix(y, x).each do |next_y, next_x|
    $count += 1
    recursive_step(next_y, next_x)
  end
end

recursive_step(start.first - 1, start.last - 1)

puts $answers.min || -1
