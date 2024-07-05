```python
from swagweb_rs import App, Config
from swagweb_rs.routing import Router, Root
from swagweb_rs.http import Request, Response, PlainTextResponse
from swagweb_rs.ctx import HTTPCtx

app = App().config(
    Config(
        listen_on="localhost:8000"
    )
)
root_router = app.router_factory.new(path=[Root])

myrouter = Router(path=["myrouter"])

                                # that ctx will be default, but for now it's here for clarity
@router.get(["say_hello_first"], ctx=HTTPCtx, 
            tasks=[lambda ctx: print(ctx.request.ip)] # just print IP
)

def say_hello_first(request: Request) -> Response:
    return PlainTextResponse(f"Leaked IP: {request.ip}")

@router.get(
    ["say_hello_tasks"], ctx=HTTPCtx,
    # Tasks output(return) will be ignored. It's just a way to execute functions in a list.
    tasks=[ lambda ctx: [i(ctx) for i in ctx.user["todo:"]] ] # just example of custom task with user-extended HTTPCtx. Will print the entire HTTPCtx two times.
)
def get(request: Request) -> Response:
    # That's how we provide additional context to the tasks. It's shared via Python dictionary. Also I would like to have something like `ctx_dict`, that will accept only one dict, without chaining ability, while just `ctx` will allow chaining as long as you want.
    return PlainTextResponse(f"Leaked IP: {request.ip}").ctx("todo:", [print, print])

app.router_factory.add(root_router.compose(myrouter))
app.run()
```
