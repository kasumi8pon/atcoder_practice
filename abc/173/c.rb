h, w, k = gets.split.map(&:to_i)
graph = []
h.times do
  graph << gets.chomp.split('')
end

black = graph.flatten.count('#')
minus = black - k

answer = 0

[0, 1].repeated_permutation(h + w) do |bits|
  tmp_graph = Marshal.load(Marshal.dump(graph))
  count = 0

  bits.each.with_index do |bit, i|
    next if bit == 0

    if i < h
      tmp_graph[i].each.with_index do |cell, num|
        if cell == '#'
          tmp_graph[i][num] = 'X'
          count += 1
        end
      end
    else
      tmp_graph.each.with_index do |row, num|
        if row[i - h] == '#'
          tmp_graph[num][i - h] = 'X'
          count += 1
        end
      end
    end

  end
  answer += 1 if count == minus
end

puts answer
