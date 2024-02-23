### Lox programming language

#### Example program: (https://benhoyt.com/writings/loxlox/)
```
for (var i = 1; i < 5; i = i + 1) {
  print i * i;
}

class Duck {
  init(name) {
    this.name = name;
  }

  quack() {
    print this.name + " quacks";
  }
}

var duck = Duck("Waddles");
duck.quack();

fun make_adder(n) {
  fun adder(i) {
    return n + i;
  }
  return adder;
}
var add5 = make_adder(5);
print add5(1);
print add5(100);

// Output:
// 1
// 4
// 9
// 16
// Waddles quacks
// 6
// 105

```


# Lox supports following things:

## Dynamic Typing
Variables can store values of any type, and a single variable can even store values of different types at different times.
## Automatic memeory management
Users don't have to manually allocate and free the memory.
## Data Types
   - Boolean (true, false)
   - Number (integer, decimal)
   - String 
   - Nil
## Expressions
   - Artihmetic
     - add + me
     - subtract - me
     - multiply - me
     - divide - me
     - -negateMe
   - Comparision and equality
     - less < than
     - greater > than
     - lessThan <= orEqual
     - greaterThan >= orEqual
   - Logical operators
     - !bool1
     - bool1 and bool2
     - bool1 or bool2
   - Precedence and grouping
     - a + b / 2 == a + (b / 2)
## Statements
   - print "Hello, World!";
## Variables
   - var iAmStr = "here is my val";
   - var iAmNil;
   - var a = "apple"; print a; a = "ball"; print b;
## Control Flow
   - if (condition) { print "yes"; } else { print "no"; }
   - for (var a = 1; a < 10; a++) { print a; }
   - var a = 1; while (a < 10) { print a; a = a + 1; }
## Functions
   - fun add(a, b) { print a + b; }
   - fun add(a, b) { return a + b; }
   - add(2, 4);
## Closures
   ```
fun returnFunction() {
        var outside = "outside";

        fun inner() {
            print outside;
        }

        return inner;
}

var fn = returnFunction();
fn();
   ```
inner() has to “hold on” to references to any surrounding variables that it uses so that they stay around even after the outer function has returned. This concept is called closure.
## Classes
```
class Breakfast {
  cook() {
    print "Eggs a-fryin'!";
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
```
## Single Inheritance
```
class Brunch < Breakfast {
  drink() {
    print "How about a Bloody Mary?";
  }
}

var benedict = Brunch("ham", "English muffin");
benedict.serve("Noble Reader");
```
```
class Brunch < Breakfast {
  init(meat, bread, drink) {
    super.init(meat, bread);
    this.drink = drink;
  }
}
```
