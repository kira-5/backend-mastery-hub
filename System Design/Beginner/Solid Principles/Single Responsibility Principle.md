# Single Responsibility Principle (SRP)

- SRP states that `a class should have only one reason to change`, meaning:
  - It should serve one primary purpose.
  - Changes to that purpose should come from one actor (e.g., only the "database team" or "reporting team").

## Core Concept

- Every module, class, or function should be responsible for a single part of the functionality
- Each should have only one reason to change
- Helps create systems that are easier to understand, maintain, and modify

## Types of Responsibilities

A responsibility can be:

- **Business Logic** (e.g., calculating order totals)
- **Persistence** (e.g., saving data to a database)
- **Networking** (e.g., sending HTTP requests)
- **Logging/Error Handling** (e.g., logging errors to a file)

## When to Apply SRP

- When a class:
  - Has >1 public methods unrelated to its core purpose.
  - Requires changes for unrelated reasons (e.g., DB schema + email format).
  - becomes too large or complex

## Examples

### ❌ Violating SRP

#### Example 1: User Class with Multiple Responsibilities

```python
class User:
    def __init__(self, name: str, email: str):
        self.name = name
        self.email = email
    
    def save_to_database(self):
        # Database logic
        print(f"Saving {self.name} to database...")
    
    def send_email(self, message: str):
        # Email sending logic
        print(f"Sending email to {self.email}: {message}")
    
    def generate_report(self):
        # Report generation logic
        print(f"Generating report for {self.name}")

# Usage
user = User("John Doe", "john@example.com")
user.save_to_database()
user.send_email("Welcome!")
user.generate_report()
```

**Problems:**

- The `User` class has multiple responsibilities:
  - Saving to database
  - Sending email
  - Generating report
- If any of these responsibilities change, the class must be modified

#### Example 2: Order Processor with Multiple Responsibilities

```python
class OrderProcessor:
    def process(self, order):
        # Validate order
        if not order.is_valid():
            raise ValueError("Invalid order")
        
        # Calculate tax
        tax = self._calculate_tax(order)
        
        # Process payment
        payment_success = self._process_payment(order, tax)
        
        if payment_success:
            # Update inventory
            self._update_inventory(order)
            
            # Send confirmation
            self._send_confirmation(order)
            
            # Save order
            self._save_order(order)
    
    def _calculate_tax(self, order):
        print("Calculating tax...")
        return order.total * 0.08
    
    def _process_payment(self, order, tax):
        print(f"Processing payment of ${order.total + tax}")
        return True
    
    def _update_inventory(self, order):
        print("Updating inventory...")
    
    def _send_confirmation(self, order):
        print("Sending confirmation email...")
    
    def _save_order(self, order):
        print("Saving order to database...")

# Usage
order = Order()  # Assume Order class exists
processor = OrderProcessor()
processor.process(order)
```

**Problems:**

- The `OrderProcessor` class has multiple responsibilities:
  - Processing orders
  - Calculating tax
  - Processing payments
  - Updating inventory
  - Sending confirmation email
  - Saving order to database
- If any of these responsibilities change, the class must be modified

### ✅ Following SRP

#### Example 1: Separated User Responsibilities

```python
class User:
    def __init__(self, name: str, email: str):
        self.name = name
        self.email = email

class UserRepository:
    @staticmethod
    def save(user: User):
        print(f"Saving {user.name} to database...")

class EmailService:
    @staticmethod
    def send_email(user: User, message: str):
        print(f"Sending email to {user.email}: {message}")

class ReportGenerator:
    @staticmethod
    def generate_user_report(user: User):
        print(f"Generating report for {user.name}")

# Usage
user = User("John Doe", "john@example.com")
UserRepository.save(user)
EmailService.send_email(user, "Welcome!")
ReportGenerator.generate_user_report(user)
```

#### Example 2: Separated Order Processing Responsibilities

```python
class OrderValidator:
    @staticmethod
    def validate(order):
        if not order.is_valid():
            raise ValueError("Invalid order")

class TaxCalculator:
    @staticmethod
    def calculate(order):
        print("Calculating tax...")
        return order.total * 0.08

class PaymentProcessor:
    @staticmethod
    def process_payment(order, amount):
        print(f"Processing payment of ${amount}")
        return True

class InventoryUpdater:
    @staticmethod
    def update(order):
        print("Updating inventory...")

class OrderConfirmation:
    @staticmethod
    def send(order):
        print("Sending confirmation email...")

class OrderRepository:
    @staticmethod
    def save(order):
        print("Saving order to database...")

class OrderProcessor:
    def __init__(self):
        self.validator = OrderValidator()
        self.tax_calculator = TaxCalculator()
        self.payment_processor = PaymentProcessor()
        self.inventory_updater = InventoryUpdater()
        self.confirmation = OrderConfirmation()
        self.repository = OrderRepository()
    
    def process(self, order):
        self.validator.validate(order)
        tax = self.tax_calculator.calculate(order)
        amount = order.total + tax
        
        if self.payment_processor.process_payment(order, amount):
            self.inventory_updater.update(order)
            self.confirmation.send(order)
            self.repository.save(order)

# Usage
order = Order()  # Assume Order class exists
processor = OrderProcessor()
processor.process(order)
```

## SRP in Different Layers of System Design

### 1. Microservices Architecture

- Each microservice should do one thing well
  - Auth Service → Only handles authentication
  - Order Service → Only manages orders
  - Notification Service → Only sends emails/SMS
- ❌ Bad: A single "UserService" handling auth, profile updates, and notifications
- ✅ Good: Split into AuthService, ProfileService, NotificationService

### 2. Database Design

- Single-table responsibility: Avoid "god tables" that store unrelated data
- Example:
  - ❌ Bad: UserTable with orders, payments, addresses
  - ✅ Good: Users, Orders, Payments, Addresses (separate tables)

### 3. API Design

- Each API endpoint should do one thing
- ❌ Bad: `/users` handling GET, POST, DELETE, profile-update
- ✅ Good: Split into:
  - GET `/users` → Fetch users
  - POST `/users` → Create user
  - PUT `/users/profile` → Update profile

## Common SRP Violations & Fixes

| Violation | Problem | Fix |
|-----------|---------|-----|
| "God Class" | Does everything (e.g., `Utils`). | Split into `StringUtils`, `DateUtils`, etc. |
| "Fat Service" | Mixes business logic, `DB`, and `I/O`. | Use Domain-Driven Design (DDD) with separate services |
| "Monolithic Functions" | A function doing 10+ steps. | Refactor into smaller functions. |

## When is it Okay to Break SRP?

- **Performance-critical code** (e.g., tightly coupled optimizations)
- **Simple scripts** (e.g., a 10-line Python script doesn't need SRP)
- **Early prototyping** (but refactor before production)

## Key Takeaways

- A class/module/service should have only one reason to change
- Split large components into smaller, focused ones
- Applies at all levels: functions, classes, APIs, microservices
- Use small classes: Each class should do one thing
- Separate business logic from I/O: Keep database, network, file operations separate
