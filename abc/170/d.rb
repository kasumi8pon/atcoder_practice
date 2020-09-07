n = gets.to_i
as = gets.split.map(&:to_i)

require 'prime'
require 'pry'

primes = Hash.new { false }
answer = 0

unless as.uniq.size == 1
  prime_divisions = as.sort.map {|a| Prime.prime_division(a) }

  pp prime_divisions

  prime_divisions.each do |division|
    checked = false

    division.each do |div|
      if primes[div.first] == true
        answer += 1 if checked == false
        checked = true
      end
      primes[div.first] = true
    end
  end
end

puts answer
