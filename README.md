This is my linear algebra library I wrote in Rust to get a little more comfortable with the language. It can handle matrix arithmetic and simple matrix operations.
There are a few limitations (like a size limit on inverting matrices) that mainly stem from the fact that I don't wanna program row reduction 😋.

I'd like to come back once I'm smarter and add more functionality and find better ways to calculate some things. In particular I'd like to implement some form of row reduction, more robust matrix inversion, orthogonalization, stuff like that.
I'll keep the *test.rs* file as an example for creating Matrix objects and ho to use each method and their expected behavior.

---
As an aside, I'm really proud of how I'm calculating determinants. Determinants are pretty difficult to calculate, so I'm taking the U of LU decomposition and taking the product of the diagonal entries. I thought it was kinda sneaky and cool so I wanted to talk about it so maybe you'd think it was cool and perhaps even consider hiring me.
