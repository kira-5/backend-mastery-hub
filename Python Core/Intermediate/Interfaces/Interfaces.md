# Python Interfaces

## Overview

An interface is a concept used to define a blueprint for a class, specifying a set of methods that must be implemented by any class that claims to adhere to that interface.

## Implementation Methods

### 1. Abstract Base Classes (ABCs)

**Characteristics:**

- Strict enforcement at compile-time
- Raises TypeError if abstract methods aren't implemented
- Less flexible but more structured

```python
from abc import ABC, abstractmethod

# Define an interface (abstract class)
class Shape(ABC):
    @abstractmethod
    def area(self):
        pass

    @abstractmethod
    def perimeter(self):
        pass

# Implement the interface
class Circle(Shape):
    def __init__(self, radius):
        self.radius = radius

    def area(self):
        return 3.14 * self.radius ** 2

    def perimeter(self):
        return 2 * 3.14 * self.radius

# Using the class
circle = Circle(5)
print(circle.area())      # Output: 78.5
print(circle.perimeter()) # Output: 31.4
```

### 2. Informal Duck Typing

**Characteristics:**

- Loose enforcement at runtime
- Raises NotImplementedError if methods aren't implemented
- More flexible approach

```python
class Shape:
    def area(self):
        raise NotImplementedError("Subclass must implement area()")

    def perimeter(self):
        raise NotImplementedError("Subclass must implement perimeter()")

class Square(Shape):
    def __init__(self, side):
        self.side = side

    def area(self):
        return self.side ** 2

    def perimeter(self):
        return 4 * self.side

square = Square(4)
print(square.area())      # Output: 16
print(square.perimeter()) # Output: 16
```

## Comparison Table

| Feature | ABC (Formal Interface) | Duck Typing (Informal) |
|---------|----------------------|----------------------|
| Enforcement | Strict (compile-time) | Loose (runtime) |
| Error Handling | TypeError on instantiation | NotImplementedError at runtime |
| Flexibility | Less flexible | More flexible |

## Key Purposes

### 1. Contractual Obligation

Ensures classes adhere to specific methods and properties.

#### Without Interface (Duck Typing)

```python
class Circle:
    def area(self):
        return 3.14 * self.radius ** 2

# Later, you realize you forgot perimeter()... but no error until runtime!
circle = Circle(5)
circle.perimeter()  # Raises AttributeError!
```

#### With Interface (ABC)

```python
from abc import ABC, abstractmethod

class Shape(ABC):
    @abstractmethod
    def area(self): pass
    @abstractmethod
    def perimeter(self): pass

class Circle(Shape):
    def area(self): 
        return 3.14 * self.radius ** 2
    # Forgot perimeter()? Error at instantiation:
circle = Circle(5)  # TypeError: Can't instantiate Circle without perimeter()
```

### 2. Polymorphism

Allows different classes to be used interchangeably if they implement the same interface.

```python
def print_stats(shape: Shape):
    print(f"Area: {shape.area()}, Perimeter: {shape.perimeter()}")

shapes = [Circle(5), Rectangle(4, 3)]
for shape in shapes:
    print_stats(shape)  # Works for ANY Shape!
```

## Best Practices

- Use ABCs when you need strict enforcement and clear contracts
- Use duck typing when you need more flexibility
- Consider your project's requirements when choosing between the two approaches

## Design Patterns

### 1. Strategy Pattern

*[To be implemented]*

### 2. Adapter Pattern

*[To be implemented]*

### 3. Observer Pattern

*[To be implemented]*

### 4. Factory Method Pattern

*[To be implemented]*

### 5. Decorator Pattern

*[To be implemented]*
