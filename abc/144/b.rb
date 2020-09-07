n = gets.to_i

answer = 'No'

9.downto(1) do |i|
  if n % i == 0 && n / i < 10
    answer = 'Yes'
    break
  end
end

puts answer
