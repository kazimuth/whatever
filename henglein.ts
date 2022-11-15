// unrelated thought:
// it occurs to me that you'd actually expect both "null" and "undefined"
// in a four valued logic: null is bottom, undefined is top. (well,
// in a "total" setting null is really throwing something...)

type ConcreteBag<T> = {
    type: "ConcreteBag",
    elements: Array<T>
}
type SumBag<T> = {
    type: "SumBag"
    addends: Bag<Bag<T>>
}
/// Fuck yeah typescript rules
/// This means ProductBag<{a: X, b: Y}> = {
///    type: "ProductBag",
///    a: Bag<X>,
///    b: Bag<Y>,
/// }
type ProductBag<Carrier> = {
    type: "ProductBag"
    (elem: keyof Carrier): Bag<typeof elem>
}

type Bag<T> = ConcreteBag<T> | SumBag<T> | ProductBag<T>

function bag<T>(...elements: Array<T>): ConcreteBag<T> {
    return { type: "ConcreteBag", elements }
}

type RawOp<I, O> = (input: I) => O;

// there's no associated types in TS
// but you can fake it with typed
// construction functions
type SequentialOp<I, O> = {
    type: "SequenceOp",
    first: Op<I, any>,
    intermediates: Op<any, any>[],
    last: Op<any, O>
}

type SubsetOfKeys<T> = {
    type: "Pattern",
    (k: keyof T): any
}

type ParallelOp<I, O extends SubsetOfKeys<I>> = {
    type: "ParallelOp",
    (k: keyof I): Op<typeof k, O[typeof k]>
}

// TODO: is this gonna annoy us w system props?
interface Pattern<T> {
    type: "Pattern"
    (k: keyof T): null
};

// TODO 
type ProjectionOp<I, O extends SubsetOfKeys<I>> = {
    type: "ProjectionOp",
    from: Pattern<I>,
    to: Pattern<O>
}


type Op<I, O> = RawOp<I, O> | SequentialOp<I, O> | ParallelOp<I, O> | ProjectionOp<I, O>


/*
// TODO: implement with iterators/generators
function flatten<T>(bag: Bag<T>): Iterator<T> {
    if (bag.type === "ConcreteBag") {
        return bag.elements[Symbol.iterator]();
    } else if (bag.type === "SumBag") {
        let elements: Array<T> = [];
        let addends = flatten(bag.addends);
        for (let bag of addends.elements) {
            for (let item of flatten(bag).elements) {
                yield item;
            }
        }
    } else if (bag.type === "ProductBag") {
        let names: string[] = [];
        let collections: any[][] = [];
        let lengths: number[] = [];
        let clock: number[] = [];
        for (let name in bag) {
            if (name === "type") continue;
            let collection = flatten(bag[name]).elements;
            names.push(name);
            collections.push(collection);
            lengths.push(collection.length);
            clock.push(0);
        }

        let elements: T[] = [];

        while (true) {
            let ptr = clock.length - 1;

            while (true) {
                clock[ptr]++;
                if (clock[ptr] > lengths[ptr]) {
                    clock[ptr] = 0;
                    ptr--;
                    if (ptr < 0) {
                        return { type: "ConcreteBag", elements }
                    }
                } else {
                    break;
                }
            }


        }

    }
}
*/