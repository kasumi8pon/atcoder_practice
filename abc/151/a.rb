c = gets.chomp
alphabets = %w(a b c d e f g h i j k l m n l p q r s t u v w x y z)

index = alphabets.find_index(c) + 1

puts alphabets[index]

