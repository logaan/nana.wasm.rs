start_time = Time.now

count = 0
while count < 1_000_000_000
  count += 1
end

end_time = Time.now
elapsed_time = end_time - start_time

puts "Time taken to count to 1 billion: #{(elapsed_time * 1000).round(0)} milliseconds"
