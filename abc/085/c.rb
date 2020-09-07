n, y = gets.split.map(&:to_i)

max_10000 = [y / 10000, n].min

answer = [-1, -1, -1]

(0..max_10000).each do |n_10000|
  remain = y - 10000 * n_10000
  max_5000 = [remain / 5000, n - n_10000].min

  (0..max_5000).each do |n_5000|
    n_1000 = n - (n_10000 + n_5000)
    if n_10000 * 10000 + n_5000 * 5000 + n_1000 * 1000 == y
      answer = [n_10000, n_5000, n_1000]
    end
  end
end

puts answer.join(' ')

def price(a, b)
  a * 10000 + b * 5000 + (n - (a + b)) * 1000
end


