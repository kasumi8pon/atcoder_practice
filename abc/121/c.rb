n, m = gets.split.map(&:to_i)
drinks = []
n.times do
  drinks << gets.split.map(&:to_i)
end

sorted_drinks = drinks.sort_by(&:first)

sum_price = 0
rest_num = m

sorted_drinks.each do |price, num|
  buying = rest_num < num ? rest_num : num

  sum_price += price * buying
  rest_num -= buying

  break if rest_num.zero?
end

puts sum_price
