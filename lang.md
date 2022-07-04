# The Rank Program Language Standard(2022-7-2)

made by ZeWei(2022-7-22)(1352677769@qq.com)
*must have a `main` function*
Benefit a lot from rust

## 1 the compliance type

### 1.1 struct

```rust
struct Person{
    name:String,
    age:u8
}
```

the size = *std::mem::sizeof::< String >()* + *std::mem::sizeof::< u8 >()*

### 1.2 union

```rust
union Uint{
    Little(u8),
    Large(usize)
}
```

the size = the bigger type 's size

### 1.3 enum(union with tag)

```rust
enum Uint{
    Little(u8),
    Large(usize)
}
```

the size equals the bigger type's size + sizeof(u8)

## 2 The basic grammar

### 2.1 The funtion

We support two different function declaration syntax.
one is like the Rust:

```rust
fn function(name:String,age:u8) -> *mut u8{
    // something interesting
}
```

the other one is this:

```rust
fn function:name,age -> *mut u8
    where name:String,age:u8
{
    //something interesting
}
```

In that case,You can omit the type mark if the compiler can get *explicit types* from the context.
We also suppost function visibility.
Of course,We still have some grammatical sugar,which needs more knowledge to introduce.

### 2.2 The Control Flow

our control flow uses standard if-else grammar like the Rust

```rust
fn main(){
    if false{
        1+1
    }else if true{
        2+2
    }else{
        3+3
    };
}
```

### 2.3 Loop

When we meet some repeat work,Maybe the loop is a good choice.We suppost for,while

```rust
while <condition>{
    // something interesting
}
```

```rust
for <var> in <Iterator>{
    // something interesting
}
```

In fact,the for is only a grammatical sugar.It will be expand into:

```rust
while let Some(<var>) = Iterator::next(&<Iterator>){
    // something interesting
}
```

### 2.4 Match

```rust
match Some(5){
    Some(n) => n,
    None => panic!(
    )
}
```
