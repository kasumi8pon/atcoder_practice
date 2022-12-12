n, second = gets.split.map(&:to_i)
playlist = gets.split.map(&:to_i)

total_second = playlist.sum
mod_second = second % total_second

song_number = nil
left_second = nil

playlist.each.with_index(1) do |song_second, i|
  mod_second -= song_second
  if mod_second <= 0
    song_number = i
    left_second = song_second + mod_second
    break
  end
end

puts [song_number, left_second].join(' ')
