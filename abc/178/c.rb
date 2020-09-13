n = gets.to_i

if n < 2
  puts 0
else
  all = 10 ** n
  include_0 = all - (9 ** n)
  include_9 = all - (9 ** n)
  include_0_or_9 = all - (8 ** n)
  puts (include_0 + include_9 - include_0_or_9) % (10 ** 9 + 7)
end
