require 'chunky_png'

HEIGHT = 100
WIDE = 20
NARROW = 10

def as_barcode(num)
  case num
  when 0
    [NARROW, NARROW, WIDE, WIDE, NARROW]
  when 1
    [WIDE, NARROW, NARROW, NARROW, WIDE]
  when 2
    [NARROW, WIDE, NARROW, NARROW, WIDE]
  when 3
    [WIDE, WIDE, NARROW, NARROW, NARROW]
  when 4
    [NARROW, NARROW, WIDE, NARROW, WIDE]
  when 5
    [WIDE, NARROW, WIDE, NARROW, NARROW]
  when 6
    [NARROW, WIDE, WIDE, NARROW, NARROW]
  when 7
    [NARROW, NARROW, NARROW, WIDE, WIDE]
  when 8
    [WIDE, NARROW, NARROW, WIDE, NARROW]
  when 9
    [NARROW, WIDE, NARROW, WIDE, NARROW]
  else
    puts 'WHAAT'
    exit
  end
end

number = ARGV[0].to_s
numbers = []
number.chars.each { |x| numbers.push x.to_i }

bars = []
# Add Start
bars.push WIDE, WIDE, NARROW
# Add middle
numbers.each { |num| as_barcode(num).each { |bar| bars.push bar } }
# Add End
bars.append WIDE, NARROW, WIDE

total_width = bars.sum + (bars.length - 1) * NARROW

img = ChunkyPNG::Image.new(total_width, HEIGHT, ChunkyPNG::Color::TRANSPARENT)

x = 0
bars.each do |bar|
  (0..HEIGHT - 1).each do |y|
    (0..bar - 1).each { |off| img[x + off, y] = ChunkyPNG::Color::BLACK }
  end
  x += bar + NARROW
end

img.save('barcode.png')
