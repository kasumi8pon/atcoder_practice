n, k = gets.split.map(&:to_i)
r, s, p = gets.split.map(&:to_i)
game_hands = gets.chomp.chars

win_hand = { 'r' => 'p', 's' => 'r', 'p' => 's' }
win_score = { 'r' => r, 's' => s, 'p' => p }

takahashi_hands = []
score = 0

(0...n).each do |i|
  hand = win_hand[game_hands[i]]
  prev_hand = i >= k ? takahashi_hands[i - k] : nil

  next if prev_hand == hand

  takahashi_hands[i] = hand
  score += win_score[hand]
end

puts score
