require 'set'

input = File.readlines('d3input.txt')


def find_nonoverlapping_claim(input) 
  fabric_size = 1000

  fabric = Array.new(fabric_size) { Array.new(fabric_size) {0}}
  nonoverlapped = Set.new();
  input.each do | line | 
    space_split = line.split(" ");
    claim_number = space_split[0][1..-1].to_i
    location = space_split[2].split(",").map(&:to_i)
    size = space_split[3].split("x").map(&:to_i)
    overlapped = false
    (location[0]...location[0]+size[0]).each do |i|
      (location[1]...location[1]+size[1]).each do |j|
        if fabric[i][j] > 0 then
          overlapped = true
          nonoverlapped.delete(fabric[i][j])
        end
        fabric[i][j] = claim_number
      end
    end
    if not overlapped then
      nonoverlapped.add(claim_number)
    end
  end
  return nonoverlapped.to_a
end

the_one = find_nonoverlapping_claim(input)
puts the_one