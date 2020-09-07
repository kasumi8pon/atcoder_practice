n = gets.to_i
chars = gets.chars

chars_with_index =
  chars.each_with_object(Hash.new{ |hash, key| hash[key] = [] }).with_index(1) do |(char, result), i|
    result[char] << i
  end
