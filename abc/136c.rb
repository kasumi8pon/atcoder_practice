n = gets.chomp.to_i
h = gets.chomp.split.map(&:to_i)

answer = "Yes"

(n - 1).downto(2) do |i|
  h[i - 1] -= 1 if h[i] < h[i - 1]
  if h[i] < h[i - 1]
    answer = "No"
    break
  end
end

puts answer
