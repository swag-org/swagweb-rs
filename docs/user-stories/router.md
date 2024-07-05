Simples example: create absolute router.
```python
from swagweb_rs import App, Config
from swagweb_rs.routing import Router, Root
from swagweb_rs.http import Request, Response, PlainTextResponse

app = App(
    config = Config(
        listen_on="localhost:8000"
    )
)

router = app.router_factory.new(path=[Root, "myrouter"]) # absolute router

# will listening at localhost:8000/myrouter/say_hello
@router.get(["say_hello"])
def get(request: Request) -> Response:
    return PlainTextResponse("Hello from swagweb!")

app.run()
```

Best practice: create root router and then compose others to it. Routers will be relative, but composed versions will be absolute.

```python
from swagweb_rs import App, Router, Config
from swagweb_rs.routing import Router, Root
from swagweb_rs.http import Request, Response, PlainTextResponse

app = App(
    config = Config(
        listen_on="localhost:8000"
    )
)
# Here we use `new` to get all the middleware defined by the application as a global.
root_router = app.router_factory.new(path=[Root]) # absolute

router = Router(path=["myrouter"]) # relative, because not starting with the `Root`

@router.get(["say_hello"])
def get(request: Request) -> Response:
    return PlainTextResponse("Hello from swagweb!")

# `.route_factory.add` allow only routers starts with the `Root` (.compose here will do that for us)
# Router.compose -> Router
app.router_factory.add(root_router.compose(router))

app.run()
```

