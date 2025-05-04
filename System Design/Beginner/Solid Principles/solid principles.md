## SOLID Principles

### Single Responsibility Principle (SRP)

- A class should have one and only one reason to change, meaning that a class should have only one job.
- Each module, class, or service should have only one reason to change (i.e., one responsibility).
- This principle helps in reducing coupling, improving maintainability, and making systems easier to refactor.
- In system design, this means:
  - Microservices should have a single responsibility (e.g., Auth Service only handles authentication).
  - Classes/functions should not mix concerns (e.g., a UserService should not also handle email notifications).
- A responsibility is a "reason to change"—if a class has multiple reasons to change, it violates SRP.

### Open/Closed Principle (OCP)

- Software entities (classes, modules, services) should be open for extension but closed for modification.
- In system design:
  - Use interfaces & abstractions (e.g., PaymentProcessor interface with multiple implementations like StripeProcessor, PayPalProcessor).
  - Follow plugin architecture (e.g., adding a new database type without changing existing code).
  - Use event-driven architecture to allow new event handlers without modifying core logic.

### Liskov Substitution Principle (LSP)

- Subtypes should be substitutable for their base types without altering correctness.
- In system design:
  - Ensure derived classes/interfaces do not violate contracts (e.g., if RedisCache and DatabaseCache implement Cache, both should behave predictably).
  - Avoid breaking client expectations (e.g., a ReadOnlyFile subclass should not allow write() operations).

### Interface Segregation Principle (ISP)

- Clients should not be forced to depend on interfaces they don’t use.
- In System Design:
  - Prefer small, focused interfaces (e.g., OrderService with placeOrder(), cancelOrder() instead of a bloated IOrder with unrelated methods).
  - In microservices, avoid monolithic APIs—split into smaller, purpose-specific endpoints.

### Dependency Inversion Principle (DIP)

- High-level modules should not depend on low-level modules; both should depend on abstractions.
- In system design:
  - Use Dependency Injection (DI) (e.g., inject DatabaseRepository into UserService via an IRepository interface).
  - Decouple components (e.g., a NotificationService should depend on an IMessageSender rather than concrete EmailSender or SMSSender).

### Examples in System Design:

- Microservices Architecture: Each service follows SRP.
- Plugin Systems: Follow OCP (e.g., payment gateways in an e-commerce system).
- Caching Layer: Follows LSP (e.g., RedisCache and MemCache interchangeable).
- API Design: Follows ISP (e.g., separate UserReadAPI and UserWriteAPI).
- Layered Architecture: Follows DIP (e.g., business logic depends on abstract repositories).