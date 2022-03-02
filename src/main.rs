use std::marker::PhantomData;
use assert_type_eq::*;

fn main() {
    assert_type_eq!(
        BubbleSort<Cons<N3, Cons<N1, Cons<N2, Nil>>>>,
        Cons<N1, Cons<N2, Cons<N3, Nil>>>);
}

type N0 = Zero;
type N1 = Succ<N0>;
type N2 = Succ<N1>;
type N3 = Succ<N2>;


// NATURAL NUMBERS

trait Nat {}

struct Zero;
struct Succ<A: Nat>(PhantomData<A>);

impl Nat for Zero {}
impl<A: Nat> Nat for Succ<A> {}


// LIST

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
    type Output = CompareNat<Num, Head>;
}

type Compare<N, Ls> = <Ls as ComputeCompare<N>>::Output;


// CONCAT 

trait ComputeConcat<A: Nat> {
    type Output: List;
}

impl<A> ComputeConcat<A> for Nil
    where A: Nat
{
    type Output = Cons<A, Nil>;
}

impl<A, Head, Tail> ComputeConcat<A> for Cons<Head, Tail>
    where A: Nat,
          Head: Nat,
          Tail: List
{
    type Output = Cons<A, Cons<Head, Tail>>;
}

type Concat<H, T> = <T as ComputeConcat<H>>::Output;


// SWAP

trait ComputeSwapAndConcat<E: Equality, Head: Nat> {
    type Output: List;
}

impl<E: Equality, Head: Nat> ComputeSwapAndConcat<E, Head> for Nil {
    type Output = Cons<Head, Nil>;
}

impl<A, Other, Head> ComputeSwapAndConcat<EQ, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<Head, Cons<A, Other>>;
}

impl<A, Other, Head> ComputeSwapAndConcat<LT, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<Head, Cons<A, Other>>;
}

impl<A, Other, Head> ComputeSwapAndConcat<GT, Head> for Cons<A, Other>
    where A: Nat,
          Other: List,
          Head: Nat
{
    type Output = Cons<A, Cons<Head, Other>>;
}

type SwapAndConcat<Eq, Hd, Ls> = <Ls as ComputeSwapAndConcat<Eq, Hd>>::Output;


// BUBBLE

trait ComputeBubble {
    type Output: List;
}

impl ComputeBubble for Nil {
    type Output = Self;
}

impl<Head, Tail> ComputeBubble for Cons<Head, Tail>
    where Head: Nat,
          Tail: List + ComputeBubble + ComputeCompare<Head>,
          <Tail as ComputeBubble>::Output: ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>
{
    type Output = SwapAndConcat<Compare<Head, Tail>, Head, Bubble<Tail>>;
}


type Bubble<Ls> = <Ls as ComputeBubble>::Output;


// HELPER TYPES

trait ComputeHead {
    type Output: Nat;
}

impl ComputeHead for Nil { type Output = Zero; }
impl<Head: Nat, Tail: List> ComputeHead for Cons<Head, Tail> { type Output = Head; }

type HeadOf<Ls> = <Ls as ComputeHead>::Output;

trait ComputeTail {
    type Output: List;
}

impl ComputeTail for Nil { type Output = Nil; }
impl<Head: Nat, Tail: List> ComputeTail for Cons<Head, Tail> { type Output = Tail; }

type TailOf<Ls> = <Ls as ComputeTail>::Output;


// SORT

trait ComputeBubbleSort {
    type Bubbled: List;
    type Output: List;
}

impl ComputeBubbleSort for Nil {
    type Bubbled = Nil;
    type Output = Nil;
}

impl<Head, Tail> ComputeBubbleSort for Cons<Head, Tail>
    where Head: Nat,
          Tail: List + ComputeBubble + ComputeCompare<Head> + ComputeConcat<Head> + ComputeBubbleSort,
          <Tail as ComputeBubble>::Output: ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>,
          <<Tail as ComputeBubble>::Output as ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>>::Output: ComputeHead,
          <<Tail as ComputeBubble>::Output as ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>>::Output: ComputeTail,
          <<<Tail as ComputeBubble>::Output as ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>>::Output as ComputeTail>::Output: ComputeBubbleSort,
          <<<<Tail as ComputeBubble>::Output as ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>>::Output as ComputeTail>::Output as ComputeBubbleSort>::Output: ComputeConcat<<<<Tail as ComputeBubble>::Output as ComputeSwapAndConcat<<Tail as ComputeCompare<Head>>::Output, Head>>::Output as ComputeHead>::Output>
{
    type Bubbled = Bubble<Cons<Head, Tail>>;
    type Output = Concat<HeadOf<Self::Bubbled>, BubbleSort<TailOf<Self::Bubbled>>>;
}

type BubbleSort<Ls> = <Ls as ComputeBubbleSort>::Output;
