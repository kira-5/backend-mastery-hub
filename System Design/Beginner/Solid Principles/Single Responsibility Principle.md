## Single Responsibility Principle

- A responsibility is a "reason to change"—if a class has multiple reasons to change, it violates SRP.
- A responsibility can be:
  - Business logic (e.g., calculating order totals).
  - Persistence (e.g., saving data to a database).
  - Networking (e.g., sending HTTP requests).
  - Logging/Error Handling (e.g., logging errors to a file).

### Example of Violating SRP:

```python
    // ❌ Bad: Mixing user authentication + email notification
    class UserService {
        public void registerUser(User user) {
            // 1. Validate user
            // 2. Save to database
            // 3. Send welcome email
            // 4. Log registration
        }
    }
```
**Problems:**

- If email logic changes, UserService must be modified.
- If database schema changes, UserService must be modified.
- Hard to test (mocks needed for DB, Email, Logging).

### Applying SRP Correctly:

```python
    // ✅ Good: Each class has one responsibility
    class UserValidator {
        public boolean isValid(User user) { ... }
    }

    class UserRepository {
        public void save(User user) { ... }
    }

    class EmailService {
        public void sendWelcomeEmail(User user) { ... }
    }

    class UserRegistration {
        private UserValidator validator;
        private UserRepository repository;
        private EmailService emailService;

        public void registerUser(User user) {
            if (validator.isValid(user)) {
                repository.save(user);
                emailService.sendWelcomeEmail(user);
            }
        }
    }
```

### SRP in Different Layers of System Design:

1. **Microservices Architecture:**
    - Each microservice should do one thing well.
        - Auth Service → Only handles authentication.
        - Order Service → Only manages orders.
        - Notification Service → Only sends emails/SMS.
    - ❌ Bad: A single "UserService" handling auth, profile updates, and notifications.
    - ✅ Good: Split into AuthService, ProfileService, NotificationService.

2. **Database Design:**
    - Single-table responsibility: Avoid "god tables" that store unrelated data.
    - Example:
        - ❌ Bad: UserTable with orders, payments, addresses.
        - ✅ Good: Users, Orders, Payments, Addresses (separate tables).

3. **API Design:**
    - Each API endpoint should do one thing.
    - ❌ /users handling GET, POST, DELETE, profile-update.
    - ✅ Split into:
        - GET /users → Fetch users.
        - POST /users → Create user.
        - PUT /users/profile → Update profile.

### Common SRP Violations & Fixes:

| **Violation** | **Problem** | **Fix** |
| --- | --- | --- |
| **"God Class"** | A class doing everything (e.g., `Utils`). | Split into smaller classes (e.g., `DateUtils`, `StringUtils`). |
| **"Fat Service"** | A service handling business logic, DB, and notifications. | Use **Domain-Driven Design (DDD)** with separate services. |
| **"Monolithic Functions"** | A function with 100+ lines doing multiple tasks. | Break into smaller functions, each doing **one thing**. |

### When is it Okay to Break SRP?

- **Performance-critical code** (e.g., tightly coupled optimizations).
- **Simple scripts** (e.g., a 10-line Python script doesn’t need SRP).
- **Early prototyping** (but refactor before production).

### Key Takeaways:

- **A class/module/service should have only one reason to change.**
- **Split large components into smaller, focused ones.**
- **SRP improves testability, maintainability, and scalability.**
- **Applies at all levels: functions, classes, APIs, microservices.**