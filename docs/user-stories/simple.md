```
from swagweb_rs import App
from swagweb_rs.http import Request, Response, TextResponse

app = App()

@app.get('/')
def get(request: Request) -> Response:
    return TextResponse('Hello, world!')


app.listen('localhost:8080')
```
