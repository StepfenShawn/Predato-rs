# Predato-rs[WIP]
A tiny but blazing fast distributed computation engine in rust. 

# TODO Features
* distributed computation
* column-based
* vectorized query
* streaming batch
* llvm-jit query runtime

# Dataframe Expression API
```rust
let ctx = Context::new();
let mut lhs = ctx.read.format("csv").load("a.csv")?;
let mut rhs = ctx.read.format("csv").load("a.csv")?;

lhs = lhs.with_expr((
    case(col("*"))
    | (col("l1") > col("l2")) >>= (lit(1))
    | (col("l2") < col("l3")) >>= (lit(3))
    | otherwise >>= lit(2)
).as_col("res1"))

lhs = lhs.with_exprs([
    (col("l1") + col("l2")).as_col("sum_12"),
    col(*).sum().as_col("all_sum")
])

// Filter
lhs /= (col("l1") > 0)
// Or
lhs = lhs.filter(col("l1") > 0)

// Join
rhs = rhs.join(lhs, how: JoinType::Left)
         .left_on(&[col("l1"), col("l2")])
         .right_on(&[col("l1"), col("l2")]);

// Group By
rhs = rhs.group_by(keys: &[col("l1"), col("l2")])
         .fmap(|&chunk| {chunk})

pipe!{rhs, show . collect . select([col("l1"), col("l2"), col("l3")])}
// Or
rhs.select([col("l1"), col("l2"), col("l3")])
   .collect()
   .show();
```


# test
```
cargo test -r
```

# benchmarks
wip