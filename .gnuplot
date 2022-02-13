n = 20

array A[n]

samples(x) = $0 > (n-1) ? n : int($0+1)
mod(x) = int(x) % n
avg_n(x) = (A[mod($0)+1]=x, (sum [i=1:samples($0)] A[i]) / samples($0))


set datafile separator ' '

plot "plot.csv" using 0:(avg_n($1)) with lines lw 5 title "red", \
    "plot.csv" using 0:(avg_n($2)) with lines lw 5 title "green", \
    "plot.csv" using 0:(avg_n($3)) with lines lw 5 title "blue"

# pause 4
# reread

pause 9999