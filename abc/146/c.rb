A, B, x = gets.split.map(&:to_i)
MAX_X = 10 ** 9

def price(n)
  A * n + B * n.to_s.size
end

cant_buy = (1..MAX_X).bsearch { |n| price(n) > x }

puts cant_buy ? cant_buy - 1 : MAX_X
