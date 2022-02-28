use std::marker::PhantomData;

fn main() {
    let _a: Bubble<
        Cons<Succ<Zero>,
            Cons<Zero,
                Cons<Succ<Succ<Zero>>,
                    Cons<Zero, Nil>
                >
            >
        >
    >;
}


// NATURAL NUMBERS DEF

trait Nat {}

struct Zero;
struct Succ<A: Nat>(PhantomData<A>);

impl Nat for Zero {}
impl<A: Nat> Nat for Succ<A> {}


// LIST DEF

trait List {}

struct Nil;
struct Cons<V, C: List>(PhantomData<(V, C)>);

impl List for Nil {}
impl<V, C: List> List for Cons<V, C> {}


// EQUALITY

trait Equality {}

struct EQ;
struct LT;
struct GT;

impl Equality for EQ {}
impl Equality for LT {}
impl Equality for GT {}

// NAT COMPARISON

trait ComputeCompareNat<Rhs> {
    type Output: Equality;
}

impl ComputeCompareNat<Zero> for Zero {
    type Output = EQ;
}

impl<A: Nat> ComputeCompareNat<Succ<A>> for Zero {
    type Output = LT;
}

impl<A: Nat> ComputeCompareNat<Zero> for Succ<A> {
    type Output = GT;
}

impl<A, B> ComputeCompareNat<Succ<B>> for Succ<A>
    where A: Nat + ComputeCompareNat<B>,
          B: Nat + ComputeCompareNat<A>
{
    type Output = <A as ComputeCompareNat<B>>::Output;
}

type CompareNat<Lhs, Rhs> = <Lhs as ComputeCompareNat<Rhs>>::Output;


// NAT - NAT FROM LIST COMPARISON

trait ComputeCompare<Rhs> {
    type Output: Equality;
}

impl<Num> ComputeCompare<Num> for Nil {
    type Output = LT;
}

impl<Head, Num, Tail> ComputeCompare<Num> for Cons<Head, Tail>
    where Head: Nat + ComputeCompareNat<Num>,
          Num: Nat + ComputeCompareNat<Head>,
          Tail: List
{
    type Output = CompareNat<Head, Num>;
}

type Compare<N, Ls> = <Ls as ComputeCompare<N>>::Output;


// SWAP

trait ComputeSwap<E: Equality, Head: Nat> {
    type Output;
}

impl<E: Equality, Head: Nat> ComputeSwap<E, Head> for Nil {
    type Output = Cons<Head, Nil>;
}

impl<A, Other, Head> ComputeSwap<EQ, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<Head, Cons<A, Other>>;
}

impl<A, Other, Head> ComputeSwap<LT, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<Head, Cons<A, Other>>;
}

impl<A, Other, Head> ComputeSwap<GT, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<A, Cons<Head, Other>>;
}

type Swap<Eq, Hd, Ls> = <Ls as ComputeSwap<Eq, Hd>>::Output;


// BUBBLE

trait ComputeBubble {
    type Output;
}

impl ComputeBubble for Nil {
    type Output = Self;
}

impl<Head, Tail> ComputeBubble for Cons<Head, Tail>
    where Head: Nat,
          Tail: List + ComputeBubble + ComputeCompare<Head>,
          <Tail as ComputeBubble>::Output: ComputeSwap<<Tail as ComputeCompare<Head>>::Output, Head>
{
    type Output = Swap<Compare<Head, Tail>, Head, Bubble<Tail>>;
}


type Bubble<Ls> = <Ls as ComputeBubble>::Output;



