n, x = gets.split.map(&:to_i)
s = gets.chomp

results = s.chars

answer = results.inject(x) { |score, result|
  if result == 'o'
    score += 1
  else
    score -= 1 unless score.zero?
  end
  score
}

puts answer
