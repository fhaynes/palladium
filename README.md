# Intro
Hey there! This README will give you an overview of Palladium.

# What is it?
Palladium is a high-level programming language intended to be run on the Iridium VM (https://gitlab.com/subnetzero/iridium). In terms of syntax, it mimics Python as much as possible. In terms of functionality, it mimics Erlang as much as possible.

## Main Function

Unlike in Python, your program must contain a function named `main`. This is optional in Python, but required in Palladium. This is where execution of your program will start. 

## Modules

A package in Palladium is defined at the directory level. When importing modules, Palladium will look at directories in the same level as the file containing the `main` function, and recurse down into directories.

### Import Statements

Import statements 

## Typing

Palladium is a strongly typed and dynamically typed language. This means that types _can_ change, but _will not_ change without being explicitly converted. For example:

```python
foo = "Test"
```

Creates a variable of type `str`. This is not a valid Palladium program:

```python
foo = "Test"
bar = 1

foo + bar
```

In many weakly typed languages, bar would be converted to "1". In Palladium, you must be explicit about this:

```python
foo = "Test"
bar = 1
bar = str(bar)

foo + bar
```

## Actors and Classes

Palladium does not support classes. It instead supports _Actors_, and enforces the Actor concurrency pattern. An Actor has the following characteristics:

1. Can only send and receive messages to other Actors
2. Only an Actor can alter it's internal state, usually in response to a message from another Actor
3. An Actor can spawn children, monitor them, and restart them if needed
4. Has a `receive(msg)` function to check for messages from other Actors

### Sample Actor

```python
class Cat:
    def receive(msg):
        print("Received: {}", msg)
```

### Actor Scheduling

Because each Actor is self-contained and shares no mutable state with other Actors, they can be scheduled across all CPU cores. This allows for easy horizontal scaling across any number of cores.

### Garbage Collection

Garbage collection is done on a per Actor basis. This allows the Iridium VM, like the BEAM VM, to avoid Stop the World pauses. 


