# Casting and Converting
=begin
This Ruby code demonstrates various type casting and conversion methods. Let's break it down step by step:

```ruby
puts 3.14.to_i
```
- `3.14.to_i` converts the float `3.14` to an integer, resulting in `3`.

```ruby
puts 3.to_f
```
- `3.to_f` converts the integer `3` to a float, resulting in `3.0`.

```ruby
puts "3.0".to_s
```
- `"3.0".to_s` converts the string `"3.0"` to a string, which is redundant since it's already a string. The result is `"3.0"`.

```ruby
puts 100 + "50".to_i
```
- `"50".to_i` converts the string `"50"` to an integer, resulting in `50`.
- `100 + 50` adds the integer `100` to the integer `50`, resulting in `150`.

```ruby
puts 100 + "50.99".to_f
```
- `"50.99".to_f` converts the string `"50.99"` to a float, resulting in `50.99`.
- `100 + 50.99` adds the integer `100` to the float `50.99`, resulting in `150.99`.

This code demonstrates how to convert between different data types in Ruby, which is useful for ensuring that operations are performed correctly based on the expected data types.

=end


puts  3.14.to_i
puts  3.to_f
puts "3.0".to_s

puts 100 + "50".to_i
puts  100 + "50.99".to_f
