```python
from swagweb_rs import App, DefaultConfig
from swagweb_rs.routing import Root, Router
from swagweb_rs.http import Request, Response, PlainTextResponse

app = App().config(
    DefaultConfig(
        listen_on="localhost:8000"
    )
)

# path argument in `.route_factory.new` must starts with the `Root`
router = app.route_factory.new(path=[Root])

# dead simple middleware. Just call a handler
@router.get([], middleware = lambda req, handler: handler(req))
def get(request: Request) -> Response:
    return PlainTextResponse("Hello, world")

app.run()
```
