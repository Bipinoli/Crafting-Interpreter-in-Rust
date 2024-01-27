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

However, if we are programming in a functional way, we can create functions for operations. The function will work by matching the type. This way if we need to support a new operation, we can just create a new function and provide the logic for all the types there. However, if we need to support a new type we have to go back and modify all the previous functions.

So, if we need to add new operations and types as needed in the future there is no clear way to provide both. This is called the expression problem. It has it's origin in the expression evaluation in compilers, hence the name.


### Visitor pattern

Consider the scenario where we want to introduce a new behaviour to classes. For example, say we have a zoo with class of animals, (birds, reptiles, etc). Now we want to introduce new features on them, for example a way to know if the animal is popular right now.
Now we could implement a isPopular() method each animal class. Popularity might depend on some external data along with the feature of an animal. So, we have to also provide that metrics to the function, which creates a dependency to the class that doesn't make too much sense. Now, what if we need to provide a new feature, say to know if the animal is better fit for another zoo and should be tranferred there. How should we do that? Should we introduce a new shouldBeTrasferred() method? Our class will explode in no time, furthermore these features are not even closly related to the animal itself. They merely consume the data of the animal to provide additional features.

This is the incarnation of expression problem. We don't want to modify the existing classes just to consume their data and add new operations on top.

This is where visitor pattern comes into the picture.

It allows us to separate the implementation of those features in a separate class of itself without having to break open the existing classes again and again.


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

This way we can just create new visitors for the new behaviour as needed. The class always calls the method for itself, we we just provide a methods for the relevant classes in the visitor and call accept() methods whenever we need the data of the classes to do something.
This is quite elegant :)

