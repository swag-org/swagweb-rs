The concept of postware is in ability to change response according to the provided context avoiding code duplicate.

```python
from swagweb_rs import App, Config
from swagweb_rs.routing import Root, Router
from swagweb_rs.http import Request, Response, PlainTextResponse

app = App().config(
    Config(
        listen_on="localhost:8000"
    )
)

# path argument in `.route_factory.new` must starts with the `Root`
router = app.route_factory.new(path=[Root])

@router.get([]) # will listening at localhost:8000
def get(request: Request) -> Response:
    return PlainTextResponse("Hello, world").postware(lambda ctx: PlainTextResponse(f"{ctx.response.text}!"))

app.run()
```

Another way to declare a postware, that will work for all responses
```python
from swagweb_rs import App, Config
from swagweb_rs.routing import Root, Router
from swagweb_rs.http import Request, Response, PlainTextResponse
from swagweb_rs.ctx import HTTPCtx

app = App().config(
    Config(
        listen_on="localhost:8000"
    )
)

# path argument in `.route_factory.new` must starts with the `Root`
router = app.route_factory.new(path=[Root])

def postware(ctx: HTTPCtx) -> PlainTextResponse:
    ctx.response.text += "!"
    return ctx.response.text

@router.get([], postware=postware) # will listening at localhost:8000
def get(request: Request) -> Response:
    return PlainTextResponse("Hello, world")

app.run()
```

