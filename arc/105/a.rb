a, b, c, d = gets.split.map(&:to_i)

sum = [a, b, c, d].sum
half = sum / 2

answer = 'No'

if sum.even?
  (1..4).each do |num|
    [a, b, c, d].combination(num) do |x|
      if x.sum == half
        answer = 'Yes'
        break
      end
    end
  end
end

puts answer
