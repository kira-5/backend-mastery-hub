## ✨ Topic Name ✨

### 🔴 Advanced

* [❌] **Custom Middleware / Routers**
    
    * FastAPI: Modular routers, custom logic in middleware
        
    * Django: Middleware with complex logic
        
* [❌] **Websockets**
    
    * FastAPI: `@app.websocket("/ws")`
        
    * Django: Channels + ASGI setup
        
* [❌] **Async Views / Background Tasks**
    
    * FastAPI: `@app.get()` with `async def`, `BackgroundTasks`
        
    * Django: `async def view`, Celery or `asgiref.sync`
        
* [❌] **Custom Authentication**
    
    * FastAPI: Custom dependency for token decoding
        
    * Django: Custom `AuthenticationBackend`
        
* [❌] **API Rate Limiting**
    
    * FastAPI: `slowapi`, `limits` package
        
    * Django: `django-ratelimit`, DRF throttling