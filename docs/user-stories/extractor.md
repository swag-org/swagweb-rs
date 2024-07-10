```python
from swagweb_rs import App
from swagweb_rs.http import Request, PlainTextResponse
from swagweb_rs.extract import Query, ContextVar, PathVar

app = App()

app.on(tracing)

@app.get("/{user_id}")
def get(
    query: Dict[str, str] = Query(),
    user_id: str = PathVar("user_id"),
    trace_id: str = ContextVar("trace_id")
):
    return PlainTextResponse(f"{trace_id} a + b = {query['result']}")

app.run("localhost:8080")
```