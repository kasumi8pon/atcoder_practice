n, s = gets.chomp.split
n = n.to_i
s = s.chars.map(&:to_sym)

answer = 0

(0...n - 1).each do |first|
  counter = Hash.new(0)
  counter[s[first]] += 1

  (first + 1...n).each do |last|
    counter[s[last]] += 1
    if counter[:A] == counter[:T] && counter[:C] == counter[:G]
      answer += 1
    end
  end
end

puts answer
