input = File.readlines('d3input.txt')

# input = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]

fabric_size = 1000

fabric = Array.new(fabric_size) { Array.new(fabric_size) {|_| 0}}

input.each do | line | 
  space_split = line.split(" ");
  claim_number = space_split[0]
  location = space_split[2].split(",").map(&:to_i)
  size = space_split[3].split("x").map(&:to_i)
  (location[0]...location[0]+size[0]).each do |i|
    (location[1]...location[1]+size[1]).each do |j|
      fabric[i][j] += 1
    end
  end
end

conflicts = fabric.reduce(0) do |row_acc, row| 
  row_acc + row.reduce(0) { |col_acc, el| col_acc + (el > 1 ? 1 : 0) }
end

puts conflicts
