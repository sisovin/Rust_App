# Variables and Data Types
=begin
Names are case-sensitive and must begin with:
  letters, _ or an underscore character.
After, may include
  letters, numbers, _
Convention says
  Start with a lowercase word, then additional words are lowercase separated by underscores
  ex. my_first_variable
=end

name = "Sisovin" # String
age = 53 # Integer
gpa = 1.69 # Float
is_tall = true # Boolean

puts "Your name is #{name}"
puts "Your name is " + name
puts "Your name is " + name + " and your age is " + age.to_s

name = "Sovanny" # Reassigning a variable
age = 52 # Reassigning a variable

puts "Your name is #{name}"
puts "Your name is " + name
puts "Your name is " + name + " and your age is " + age.to_s

