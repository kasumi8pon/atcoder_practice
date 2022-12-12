s = gets.chomp

if  s.match?(/\A[A-Z][1-9]\d{5}[A-Z]\Z/)
  puts 'Yes'
else
  puts 'No'
end
