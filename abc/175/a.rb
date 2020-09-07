weathers = gets.chomp.split('')

rainy = weathers.count('R')

case rainy
when 0
  puts 0
when 1
  puts 1
when 2
  if weathers[1] == 'R'
    puts 2
  else
    puts 1
  end
when 3
  puts 3
end
