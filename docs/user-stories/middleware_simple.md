Response middleware for some specific cases.
```python
from swagweb_rs import App, Router, DefaultConfig
from swagweb_rs.routing import Root
from swagweb_rs.http import Request, Response, PlaintTextResponse

app = App().config(
    DefaultConfig(
        listen_on="localhost:8000"
    )
)

# path argument in `.route_factory.new` must starts with the `Root`
router = app.route_factory.new(path=[Root])

@router.get([]) # will listening at localhost:8000
def get(request: Request) -> Response:
    return PlainTextResponse("Hello, world").middleware(lambda ctx: f"{ctx.response.text}!")

app.run()
```
