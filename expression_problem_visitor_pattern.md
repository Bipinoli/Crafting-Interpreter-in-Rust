## Expression problem and visitor pattern

### Expression problem
Say that we have some types and operations on those types. We want all the types to have all the operations. How can we do that?


```
  Object Oriented                     Functional
────────────────────                ────────────────


         Operations                       Operations

          x   y   z                        x   y   z
 Types                            Types

       ┌────────────┐                    ┌───┬───┬───┐
  A    │            │              A     │   │   │   │
       ├────────────┤                    │   │   │   │
  B    │            │              B     │   │   │   │
       ├────────────┤                    │   │   │   │
  C    │            │              C     └───┴───┴───┘
       └────────────┘



Class A {                           fn x() {
      x() { .. }                        match {
      y() { .. }                           case A => ...
      z() { .. }                           case B => ...
}                                          case C => ...
                                        }
Class B {                           }
      x() { .. }                    fn y() {
      y() { .. }                        match {
      z() { .. }                           case A => ...
}                                          case B => ...
                                           case C => ...
                                        }
                                    }
```


If we are programming in an object oriented way, we can have bunch of classes for types and methods for the operations. When we need to add a new type, we can just create a new class and implement the methods for operations. However, if we have to support new operation we have to go back and edit all the classes to insert a new operation.

However, if we are programming in a functional way, we can create functions for operations. The function will work by matching the type. This way if we need to support a new operation, we can just create a new function and provide the logic for all the types there. However, if we needed to support a new type we have to go back and modify all the previous functions.


No matter what we choose, when we need to support a new thing, we might have to go back and edit the ones we already built. This makes things brittle. We want to be able to add new things without having to worry about modifying the old things. 
This is called an expression problem. It has it's origin in expression evaluation in compilers, hence the name.

It is a fundamental conflict in modeling the growth in a different dimension.


However, there is a way to deal with this problem in object oriented way using a visitor pattern.


### Visitor pattern

Visitor pattern solves expression problem by outsourcing the implementation of operations.

```
                                        interface Visitor {
Class A {                                    forA();
   accept(Visitor v) {                       forB();
       v.forA(this)                          forC();
   }                                    }
}                                              ▲    ▲
                                               │    │
Class B {                              ┌───────┘    └───────┐
   accept(Visitor v) {                 │                    │
       v.forB(this)                    │                    │
   }
}                                 x : Visitor {        y : Visitor {

Class C {                           forA() {             forA() {
   accpet(Visitor v) {                ...                  ...
       v.forC(this)                 }                    }
   }
}                                   forB() {             forB() {
                                      ...                  ...
                                    }                    }

                                    forC() {             forC() {
                                      ...                  ...
                                    }                    }
                                  }                    }
```

We need to add a method to each type to accept a visitor but after that, to support a new operation, we don't need to modify the types. We just create a new visitor for the particular operation and implement a method for that particular type.

For each operation we create separate methods for types, so in a way it is similar to the type matching in functional programming. So we are emulating the benefit of functional programming way in our object-oriented model.

This way we can handle the addition of both new types and new operations without having to modifiy the existing code again and again.

