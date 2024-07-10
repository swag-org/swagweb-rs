```python
from swagweb_rs import App
from swagweb_rs.routing import Router
from swagweb_rs.http import Request, PlainTextResponse

app = App()

router = Router("/router")

@router.get("/say_hello")
def get(request: Request):
    return PlainTextResponse("Hello from swagweb!")

app.compose(router)
app.run()
```
```python
from swagweb_rs import App
from swagweb_rs.routing import Router
from swagweb_rs.http import Request, PlainTextResponse, Context

app = App()

router = Router()

def tracing(ctx: Context):
    trace_id = next_trace_id()
    print(f"Request {trace_id} started")
    ctx.var("trace-id", trace_id)
    ctx.next()
    print(f"Request {trace_id} finished")

router.on(tracing)

authenticated_router = Router("/safe")

def authenticated(ctx: Context):
    ...

authenticated_router.on(authenticated)

@authenticated_router.get("/say_hello")
def get(request: Request):
    return PlainTextResponse("Hello from swagweb!")

router.compose(authenticated_router)

app.compose(router)
app.run()
```