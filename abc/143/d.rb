n = gets.to_i
lines = gets.split.map(&:to_i)

sorted_lines = lines.sort

answer = 0

(0...n - 2).each do |a|
  (a + 1...n - 1).each do |b|
    max = sorted_lines[a] + sorted_lines[b] - 1
    over_index = sorted_lines.bsearch_index { |c| c > max }

    if over_index
      answer += over_index - b - 1
    else
      answer += n - b - 1
    end
  end
end

puts answer
