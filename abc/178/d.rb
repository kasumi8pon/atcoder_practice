s = gets.to_i

if s < 3
  puts 0
else
  size = s / 3
  answer = 1
  (2..size).each do |n|
    rest = s - (3 * n)
    if rest.zero?
      answer += 1
    else
      a = n + rest - 1
      b = rest
      answer += (a - b + 1..a).inject(1, :*) / (1..b).inject(1, :*)
    end
  end

  puts answer % (10 ** 9 + 7)
end
