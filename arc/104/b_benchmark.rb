require 'benchmark'

Benchmark.bm do |x|
  n = '5000'
  s = 'AT' * 2500

  x.report {
    n = n.to_i
    s = s.chars

    answer = 0

    (0...n - 1).each do |first|
      counter = Hash.new(0)
      counter[s[first]] += 1

      (first + 1...n).each do |last|
        counter[s[last]] += 1
        if counter['A'] == counter['T'] && counter['C'] == counter['G']
          answer += 1
        end
      end
    end
  }

  n = '5000'
  s = 'AT' * 2500

  x.report {
    n = n.to_i
    s = s.chars.map(&:to_sym)

    answer = 0

    (0...n - 1).each do |first|
      counter = Hash.new(0)
      counter[s[first]] += 1

      (first + 1...n).each do |last|
        counter[s[last]] += 1
        if counter[:A] == counter[:T] && counter[:C] == counter[:G]
          answer += 1
        end
      end
    end
  }
end

# ❯ ruby arc/104/b.rb
#        user     system      total        real
#    3.077603   0.004862   3.082465 (  3.098469)
#    1.767028   0.002098   1.769126 (  1.774524)
