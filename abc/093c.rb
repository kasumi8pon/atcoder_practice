abc = gets.chomp.split(" ").map(&:to_i)
count = 0

if abc.max - abc.min >= 2
  while abc.max - abc.min >= 2
    count += 1
    abc.sort!
    abc[0] += 2
  end
elsif abc.max - abc.min == 1
  if abc.count(abc.max) == 2
    count += 2
  elsif abc.count(abc.max) == 1
    count += 1
  end
end

# while abc.max - abc.min >= 2
#   count += 1
#   abc.sort!
#   abc[0] += 2
# end
# if abc.count(abc.max) == 2
#   count += 2
# elsif abc.count(abc.max) == 1
#   count += 1
# end

puts count



# unless abc.count(abc.max) == 3
#   until abc.max - abc.min < 2
#     count += 1
#     abc.sort!
#     abc[0] += 2
#   end
#   if abc.count(abc.max) == 2
#     count += 2
#   elsif abc.count(abc.max) == 1
#     count += 1
#   end
# end