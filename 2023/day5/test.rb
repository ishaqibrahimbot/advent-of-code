
limit = 2000000000

time_before = Time.now

x = 0
for i in 0..limit
    x +=1 
end

time_after = Time.now


puts "Time: #{time_after - time_before}"