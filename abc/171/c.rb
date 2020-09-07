n = gets.to_i

tmp = 0

alphabets = ('a'..'z').to_a

# digits = (1..11).to_a.each_with_object([]) { |num, result|
#   tmp += 26 ** num
#   result << tmp
# }

# name_digit = 1

# digits.each_with_index  do |digit, i|
#   if n <= digit
#     name_digit += i
#     break
#   end
# end

answer = ''

# n - digits[name_digit - 1]

tmp_n = n

until tmp_n.zero?
  tmp_n -= 1
  q, r = tmp_n.divmod(26)
  answer << alphabets[r]
  tmp_n = q
end

pp answer.reverse
