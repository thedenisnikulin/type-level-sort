fn main() {
    let a: Cons<Zero, Cons<Succ<Zero>, Nil>>;
    // let b = a.swap();
    // let c = a.rec();

    // let d: Succ<Zero>;
    // let e: Succ<Succ<Zero>>;
    // let f = d.cmp(e);
}



// numbers

trait Nat {}

struct Zero;
struct Succ<T: Nat>(T);

impl Nat for Zero {}
impl<T: Nat> Nat for Succ<T> {}


// arrays

trait Arr {}

struct Nil;
struct Cons<V: Nat, A: Arr>(V, A);

impl Arr for Nil {}
impl<V: Nat, A: Arr> Arr for Cons<V, A> {}



// bubble

trait ComputeBubble {
    type Output;
}


impl<A, Other> ComputeBubble for Cons<A, Other>
    where A: Nat + ComputeCompare<Other>,
          Other: Arr + ComputeBubble + ComputeCompare<A>,
          Other: ComputeSwap<Compare<Other, A>>,
          <Other as ComputeBubble>::Output: ComputeSwap<<Other as ComputeCompare<A>>::Output>
{
    type Output = Swap<Compare<Other, A>, Bubble<Other>>;
}

impl ComputeBubble for Nil { type Output = Self; }

type Bubble<T> = <T as ComputeBubble>::Output;


fn aboba() {
    let t: <Cons<Zero, Cons<Succ<Zero>, Nil>> as ComputeBubble>::Output;
    let b: <Cons<Zero,
                Cons<Succ<Zero>,
                    Cons<Succ<Succ<Zero>>,
                        Cons<Succ<Zero>, Nil>>>> as ComputeBubble>::Output;
}

// 4<2<6<1<3<T>>>>>
// [ 4, 2, 6, 1, 3 ]
// first recursion: n - 1
// second recursion: swap until end


// concatenation




// comparison

trait ComputeCompare<Rhs> {
    type Output: Equality;
}

// impl ComputeCompare<Zero> for Zero             { type Output = EQ; }
// impl<A: Nat> ComputeCompare<Zero> for Succ<A>  { type Output = GT; }
// impl<A: Nat> ComputeCompare<Succ<A>> for Zero  { type Output = LT; }
// impl<A: Nat, B: Nat> ComputeCompare<Succ<B>> for Succ<A>
//     where A: ComputeCompare<B>                 { type Output = <A as ComputeCompare<B>>::Output; }

type Compare<Lhs, Rhs> = <Lhs as ComputeCompare<Rhs>>::Output;

struct EQ;
struct LT;
struct GT;

trait Equality {}
impl Equality for EQ {}
impl Equality for LT {}
impl Equality for GT {}


// Nat and Arr comparison
impl<Other: Arr>            ComputeCompare<Cons<Zero, Other>> for Zero { type Output = EQ; }
impl<A: Nat, Other: Arr>    ComputeCompare<Cons<Succ<A>, Other>> for Zero { type Output = LT; }
impl<A: Nat, Other: Arr>    ComputeCompare<Cons<Zero, Other>> for Succ<A> { type Output = GT; }

impl<A, B, Other> ComputeCompare<Cons<Succ<B>, Other>> for Succ<A>
    where A: Nat + ComputeCompare<B>,
          B: Nat + ComputeCompare<A>,
          Other: Arr
{ type Output = <A as ComputeCompare<B>>::Output; }

impl<A> ComputeCompare<Nil> for A { type Output = LT; }


// swap

trait ComputeSwap<E: Equality> {
    type Output;
}


impl<A: Nat, B: Nat, Other: Arr> ComputeSwap<EQ> for Cons<A, Cons<B, Other>> { type Output = Cons<A, Cons<B, Other>>; }
impl<A: Nat, B: Nat, Other: Arr> ComputeSwap<GT> for Cons<A, Cons<B, Other>> { type Output = Cons<A, Cons<B, Other>>; }
impl<A: Nat, B: Nat, Other: Arr> ComputeSwap<LT> for Cons<A, Cons<B, Other>> { type Output = Cons<B, Cons<A, Other>>; }
impl<A: Nat, E: Equality> ComputeSwap<E> for Cons<A, Nil> { type Output = Self; }

impl<E: Equality> ComputeSwap<E> for Nil { type Output = Nil; }

type Swap<E, T> = <T as ComputeSwap<E>>::Output;

// function bs(arr) {
//     if (arr.length == 0) {
//         return arr;
//     }
//
//     console.log("arr: " + arr);
//
//     let head = arr.slice(0, 1);
//     let tail = bs(arr.slice(1));
//      swap(head, tail);
//
//     return head.concat(tail);
// }
//
// function swap(prep, arr) {
//     if (arr.length == 0) return;
//     if (prep[0] <= arr[0]) {
//         let temp = prep[0];
//         prep[0] = arr[0];
//         arr[0] = temp;
//     }
// }
// // <head as Concat< <tail as Bubble>::Output >>::Output
