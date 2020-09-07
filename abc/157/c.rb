n, m = gets.split.map(&:to_i)
conditions = []
m.times do
  conditions << gets.split.map(&:to_i)
end

number = 'X' * n
answer = 0

conditions.each do |c|
  if number[c[0] - 1] != 'X' && number[c[0] - 1] != c[1].to_s
    answer = -1
    break
  end
  number[c[0] - 1] = c[1].to_s
end

answer = -1 if number[0] == '0' && n != 1

unless answer == -1
  number = 
    number.chars.map.with_index(0) do |c, i|
      next c if c != 'X'

      (i == 0 && n != 1) ? 1 : 0
    end
end

puts answer == -1 ? answer : number.join
