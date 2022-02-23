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

trait Bubble {
    type Output;
}


impl<A, B, Other> Bubble for Cons<A, Cons<B, Other>>
    where A: Nat + Compare<B>,
          B: Nat + Compare<A>,
          Other: Arr,
          Cons<A, Cons<B, Other>>: Swap<<A as Compare<B>>::Output> // consider to remove
{
    // type Output =
    //     <Self as Swap<
    //         <A as Compare<B>>::Output>
    //     >::Output; // return b ?

    // implement Cons<B, Other> somehow
    type Output = < <Cons<B, Other> as Bubble>::Output as Swap<<A as Compare<B>>::Output> >::Output;
}

impl<A> Bubble for Cons<A, Nil> where A: Nat { type Output = Self; }


fn aboba() {
    let t: <Cons<Zero, Cons<Succ<Zero>, Nil>> as Bubble>::Output;
    let b: <Cons<Zero,
                Cons<Succ<Zero>,
                    Cons<Succ<Succ<Zero>>,
                        Cons<Succ<Zero>, Nil>>>> as Bubble>::Output;
}

// 4<2<6<1<3<T>>>>>
// [ 4, 2, 6, 1, 3 ]
// first recursion: n - 1
// second recursion: swap until end



// comparison

trait Compare<Rhs: Nat> {
    type Output: Equality;
}

impl Compare<Zero> for Zero             { type Output = EQ; }
impl<A: Nat> Compare<Zero> for Succ<A>  { type Output = GT; }
impl<A: Nat> Compare<Succ<A>> for Zero  { type Output = LT; }
impl<A: Nat, B: Nat> Compare<Succ<B>> for Succ<A>
    where A: Compare<B>                 { type Output = <A as Compare<B>>::Output; }

struct EQ;
struct LT;
struct GT;

trait Equality {}
impl Equality for EQ {}
impl Equality for LT {}
impl Equality for GT {}


// swap

trait Swap<E: Equality> {
    type Output;
}


impl<A: Nat, B: Nat, Other: Arr> Swap<EQ> for Cons<A, Cons<B, Other>> { type Output = Cons<A, Cons<B, Other>>; }
impl<A: Nat, B: Nat, Other: Arr> Swap<GT> for Cons<A, Cons<B, Other>> { type Output = Cons<A, Cons<B, Other>>; }
impl<A: Nat, B: Nat, Other: Arr> Swap<LT> for Cons<A, Cons<B, Other>> { type Output = Cons<B, Cons<A, Other>>; }
