
struct Person {
    age: i32
}

fn main() {
    move_scenarios();
    ref_scenarios();
}

fn ref_scenarios() {
    // Reference Scenario 1: ReadOnly value -> referred from -> ReadOnly refs
    // Step i)  P owns a value as ReadOnly.
    // Step ii) Multiple ReadOnly refs to var are created
    {
        let p = Person { age: 30 };

        let ref_r1: &Person = &p;
        let ref_r2: &Person = &p;

        // ERROR: ref_r is ReadOnly
        // ref_r1.age += 1;

        println!("{}", p.age);

        println!("{}, {}, {}", p.age, ref_r1.age, ref_r2.age);
    }

    // ERROR: Reference Scenario 2: ReadOnly value -> referred from -> ReadWrite refs
    // Step i)  P owns a value as ReadOnly.
    // Step ii) ERROR: ReadWrite ref can't be created to the value
    {
        let p = Person { age: 30 };

        // ERROR: p cannot be referred to in ReadWrite fashion
        // let ref_rw = &mut p;

    }

    // Reference Scenario 3: ReadWrite value -> referred from -> ReadOnly refs
    // Step i) P owns a value as ReadWrite
    // Step ii) Multiple ReadOnly refs to var are created
    {
        let mut p = Person { age: 30 };

        // Multiple ReadOnly refs can co-exist
        let ref_r1: &Person = &p;
        let ref_r2: &Person = &p;
        func(&p);

        // ERROR: ref_r1 is ReadOnly
        // ref_r1.age += 1;

        // ERROR: p cannot be modified as it is still borrowed by refs
        // p.age += 1;

        // ERROR: p cannot be moved out of, as refs exist
        // let q = p;

        println!("{}, {}, {}", p.age, ref_r1.age, ref_r2.age);

        // p can now be modified as refs can be expired
        p.age += 1;

        // p can now be moved out of as refs can be expired
        let q = p;
        println!("{}", q.age);
    }

    // Reference Scenario 4: ReadWrite value -> referred from -> (single) ReadWrite ref
    // Step i)  P owns a value as ReadWrite.
    // Step ii) A (single) ReadWrite ref is created to the value
    {
        let mut p = Person { age: 30 };

        let ref_rw: &mut Person = &mut p;
        // Alternately:
        // fn_readwrite_ref(&mut p);

        // ERROR: Cannot have multiple ReadWrite refs simultaenously
        // let ref_rw2 = &mut p;

        ref_rw.age += 1;
        println!("{}", ref_rw.age);

        let ref_rw_to_ref_rw: &mut Person = ref_rw;

        // ERROR: Cannot have multiple ReadWrite refs simultaneously
        // let ref_rw2_to_ref_rw: &mut Person = ref_rw;

        // ERROR: Identifier 'ref_rw' is not available as RW refs exist
        // println!("{}", ref_rw.age);

        ref_rw_to_ref_rw.age += 1;
        println!("{}", ref_rw_to_ref_rw.age);

        // Identifier 'ref_rw' is now available as RW refs can be expired
        println!("{}", ref_rw.age);
    }

}

fn func(p: &Person) {
    println!("{}", p.age);
}

fn move_scenarios() {
    // Move Scenario 1: ReadOnly value -> moved as -> ReadOnly value
    // Step i)  P owns a value as ReadOnly.
    // Step ii) value is moved in ReadOnly fashion
    {
        let p = Person { age: 30 };

        let move_r: Person = p; // Alternately: fn_readonly_move(p)

        // ERROR: Identifier 'x' is no longer available
        // println!("{}", p.age);

        println!("{}", move_r.age);

        // ERROR: move_r is ReadOnly
        // move_r.age += 1;

        let mut move_rw = move_r;
        move_rw.age += 1;
        println!("{}", move_rw.age);
    }

    // Move Scenario 2: ReadOnly value -> moved as -> ReadWrite value
    // Step i)  P owns a value as ReadOnly.
    // Step ii) value is moved in ReadWrite fashion
    {
        let p = Person { age: 30 };
        let mut move_rw: Person = p; // Alternately: fn_readwrite_move(p);

        // ERROR: Identifier 'p' is no longer available
        // println!("{}", p.age);

        move_rw.age += 1;
        println!("{}", move_rw.age);
    }

    // Move Scenario 3: ReadWrite value -> moved as -> ReadOnly value
    // Step i)  P owns a value as ReadWrite.
    // Step ii) value is moved in ReadOnly fashion
    {
        let mut p = Person { age: 30 };
        p.age += 1;
        println!("{}", p.age);
        let move_r: Person = p; // Alternately: fn_readonly_move(p);

        // ERROR: Identifier 'p' is no longer available
        // println!("{}", p.age);

        // ERROR: move_r is ReadOnly
        // move_r.age += 1;

        println!("{}", move_r.age);
    }

    // Move Scenario 4: ReadWrite value -> moved as -> ReadWrite value
    // Step i)  P owns a value as ReadWrite.
    // Step ii) value is moved in ReadWrite fashion
    {
        let mut p = Person { age: 30 };
        p.age += 1;
        println!("{}", p.age);

        let mut move_rw: Person = p; // Alternately: fn_readwrite_move(p);

        // ERROR: Identifier 'p' is no longer available
        // println!("{}", p.age);

        println!("{}", move_rw.age);
        move_rw.age += 1;
        println!("{}", move_rw.age);
    }

}

fn fn_readonly_move(p: Person) {
    println!("{}", p.age);

    // ERROR: s is read only
    // s.make_ascii_uppercase();

}

fn fn_readwrite_move(mut p: Person) {
    p.age += 1;
    println!("{}", p.age);
}

fn fn_readonly_ref(p: &Person) {
    println!("{}", p.age);

    // ERROR: p is read only
    // p.age += 1;
}

fn fn_readwrite_ref(p: &mut Person) {
    println!("{}", p.age);
    p.age += 1;
}
