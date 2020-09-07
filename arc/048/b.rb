INF = 100_000
$n = gets.to_i
players = []
rating_counts = Array.new(INF + 1) { 0 }
$hand_counts = Array.new(INF + 1) { Hash.new { 0 } }

class Player
  def initialize(rating, hand)
    @rating = rating
    @hand = hand
  end

  def win
    same_rating_win + diffrent_rating_win
  end

  def even
    $hand_counts[@rating][@hand] - 1
  end

  def lose
    $n - 1 - (win + even)
  end

  def same_rating_win
    $hand_counts[@rating][(@hand % 3) + 1]
  end

  def diffrent_rating_win
    $cumulative_rating_counts[@rating - 1]
  end
end

$n.times do
  rating, hand = gets.split.map(&:to_i)
  players << Player.new(rating, hand)
  rating_counts[rating] += 1
  $hand_counts[rating][hand] += 1
end

$cumulative_rating_counts = rating_counts.each_with_object([]) { |count, result|
  result << (result.last || 0) + count
}

puts players.map { |player| [player.win, player.lose, player.even].join(' ') }
