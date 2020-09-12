n = gets.to_i
s = gets.chomp.chars

tally = s.tally
tally.default = 0
answer = tally['R'] * tally['G'] * tally['B']

(0...n).each do |i|
  (1..n).each do |d|
    j = i + d
    k = i + 2 * d
    break if k >= n

    if s[i] != s[j] && s[j] != s[k] && s[k] != s[i]
      answer -= 1
    end
  end
end

puts answer

#  1 <= n <= 4000
# 三重ループを二重ループにする問題
# 1つめの条件を満たす値が RGB の掛け算の結果だと気づけないと、引き算にたどり着くことができない
