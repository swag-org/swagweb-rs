```python
from swagweb_rs import App
from swagweb_rs.http import Request, PlainTextResponse

app = App()

@app.get("/")
def get(request: Request):
    return PlainTextResponse("Hello, world!")

app.run("localhost:8080")
```
```python
from swagweb_rs import App
from swagweb_rs.http import Context, Request, PlainTextResponse

app = App()

def printing(ctx: Context):
    print("Entered")
    ctx.next()
    print("Exited")

def with_trace_id(ctx: Context):
    trace_id = next_trace_id()
    ctx.var("trace-id", trace_id) 
    ctx.next()

def authenticated(ctx: Context):
    if "Authorization" in ctx.request.headers:
        ctx.next()
    else:
        ctx.write(PlainTextResponse("Nope.", status = 401))

app.on(printing, with_trace_id)

@app.get("/")
def get(request: Request):
    return PlainTextResponse("Hello, world!")

@app.post("/auth", also = [authenticated])
def post(request: Request):
    return PlainTextResponse("Hey!")

app.run("localhost:8080")
```