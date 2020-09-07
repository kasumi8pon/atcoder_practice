A, B, n = gets.split.map(&:to_i)

def f(x)
  (A * x / B).floor - A * (x / B).floor
end

x =
  if n < B - 1
    n
  else
    B - 1
  end

puts f(x)
