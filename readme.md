# Pi Tetration

This project came from a funny idea of Matt Parker : how to compute `π^π^π^π` ?
If you have never heard of it, I strongly suggest watching [his video](https://www.youtube.com/watch?v=BdHFLfv-ThQ), as it is highly entertaining.

Long story short, it's hard. So as an alternative we will compute the digits of `3^3^3^3` !! And having an engineering background, in my opinion `π=3` doesn't seem that wrong, right ? The only problem is that the result is a really big number, so big storing it is hard, and we want to keep it simple. So instread, we can compute its last digits. In the video, matt showed that `3^3^3^3` ends in `..6,100,739,387`. But why stop here ? what if I want to compute the digits before that ??

This program is here for you to ask ANY amount of digit (as long as it can fit on your computer). This way, you can learn that `3^3^3^3` ends in `..5,344,828,628,021,555,146,929,939,999,502,212,249,640,012,905,650,177,570,718,344,711,077,047,886,315,075,206,738,945,776,100,739,387`.

Run the command `cargo run --release -- -d 100` to get 100 digits, and `cargo run --release -- -h` to get help about the executable (yes, this is useless, but it's fun).