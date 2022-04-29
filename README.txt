Usage: cargo run <input file> <e/d> <key>
Instructions: Write any message you want in the input file to be encoded or decoded using the Vigenère cipher.
I used test.txt as my input file if you would also like to use it as well, but any text file can be used.
"e" indicates encoding the input file while "d" indicates decoding the input file. All encoded messages will be
outputted to translate.txt. Lastly, the key will be provided by the user to encode which can be anything the user wants. However,
when decoding the key provided must be the appropriate key used when it was encoded. For example, if you
ask the program to encode a file using the key: "puppy", it must use the same key, "puppy" to decode.
_____________________________________________________________________________________________________________________________________________________________

Reflection on the project:
This project enforced several of the concepts I was learning in class such as lazy or eager evaluation. Since Rust does eager evaluation, I had to adjust my 
logic for certain cases. For example, my decode method needed to loop through the vector of punctuation along with the encoded message which I was going to do
with a while loop. However, the eager evaluation caused it to evaluate outside of the bounds since I needed it to check if the index was in bounds at the same time as
evaluating the vector at the index. This was solved by using an additional if statement in the for loop. I also learned a lot about how to use Rust itself and its readability
and writability. At first, it was a little confusing to see i32 and i64 being used to represent integers as well as the different str, String, &str, etc. and applying it to 
the best of my understanding. This language is also very clear on whether certain values are mutable which gives that flexibility to the writer as well as clarity reading 
the code. In addition to that, I learned a lot more about memory management with Rust. While making functions, it is important to realize if you are passing by value or 
by reference because passing by value will move it to where it was passed in, making it inaccessible if you need to use it again. I wanted to make sure that this project 
gave me an extra challenge by trying to preserve the message's punctuation into the encode/decode while not disrupting the established cipher. I am pretty satisfied with
my project overall and I hope that it will be useful!