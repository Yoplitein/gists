import collections, select, socket, time, types

_transient_callbacks = collections.deque()
_timed_callbacks = []
_blocked_callbacks = []
_now = time.time()
_poll = select.epoll()

class _Sleep:
    __slots__ = ["delay"]
    
    def __init__(self, delay):
        self.delay = delay

class _Block:
    __slots__ = ["fd", "forRead"]
    
    def __init__(self, fd, forRead):
        self.fd = fd
        self.forRead = forRead

@types.coroutine
def _wait_readable(fd):
    _set_poll(fd, True, False)
    yield _Block(fd, True)
    _set_poll(fd, False, False)

@types.coroutine
def _wait_writable(fd):
    _set_poll(fd, False, True)
    yield _Block(fd, False)
    _set_poll(fd, False, False)

def _set_poll(fd, read = False, write = False):
    _poll.modify(fd, select.EPOLLERR | select.EPOLLHUP | (select.EPOLLIN if read else 0) | (select.EPOLLOUT if write else 0))

def run_loop(stopWhenDone = False):
    "Run the event loop indefinitely."

    while True:
        sleepTime, done = step_loop(True)
        
        if stopWhenDone and done:
            break

def step_loop(blocking = False):
    "Run a single iteration of the event loop, putting the program to sleep if blocking is true."

    global _now
    
    def reverse_iter(what):
        return reversed(list(enumerate(what)))
    
    _now = time.time()
    
    ## step 1: dispatch timer-based callbacks
    for index, (readyTime, cb) in reverse_iter(_timed_callbacks):
        if readyTime <= _now:
            cb()
            _timed_callbacks.pop(index)
    
    ## step 2: dispatch transient (one-shot) callbacks
    for _ in range(len(_transient_callbacks)): # current callbacks may queue subsequent callbacks
        _transient_callbacks.popleft()()
    
    ## step 3: poll which file descriptors are ready; sleeping if requested, and there are none
    _now = time.time() # just in case transients took unusually long (e.g. computation-heavy tasks)
    maxSleepTime = ( # the shortest duration the application should sleep for before any timers need to be dispatched
        max(0, min(pair[0] for pair in _timed_callbacks) - _now)
            if len(_timed_callbacks) > 0 else -1
    )
    readySet = {pair[0]:pair[1] for pair in _poll.poll(maxSleepTime if blocking else 0)}
    
    ## step 4: wake any waiting tasks if their fds are ready
    for index, (fd, reading, cb) in reverse_iter(_blocked_callbacks):
        readyFlags = readySet.get(fd, None)
        
        if readyFlags is None:
            continue
        
        if (reading and readyFlags & select.EPOLLIN) or (not reading and readyFlags & select.EPOLLOUT):
            cb()
            _blocked_callbacks.pop(index)
    
    return (
        ((_now + maxSleepTime) - time.time()) # the amount of time calling code should sleep when blocking = False
            if maxSleepTime > 0 else 0,
        not any(len(v) > 0 for v in [_transient_callbacks, _timed_callbacks, _blocked_callbacks]) # whether any tasks are still scheduled, i.e. if the application is done
    )

def call_soon(fn, *args, **kwargs):
    "Schedule the given function to run immediately on the next iteration of the event loop."
    
    _transient_callbacks.append(lambda: fn(*args, **kwargs))

def call_later(delay, fn, *args, **kwargs):
    "Schedule the given function to run after a delay."
    
    _timed_callbacks.append((_now + delay, lambda: fn(*args, **kwargs)))

def spawn_task(coro):
    "Schedules execution of the given coroutine within the event loop, conceptually spawning a new thread."

    assert type(coro) is types.CoroutineType, f"spawn_task got unexpected {coro!r}"
    
    def resume():
        "Helper function to execute & re-schedule the task"
        
        res = None
        try:
            res = coro.send(None) # call back into the coroutine, getting whatever was yielded this time
        except StopIteration:
            return # coroutine exited normally
        
        if res is None: # special case for sleep, also handy generally to return to the event loop during expensive computation in a coro (preventing starvation)
            call_soon(resume)
        elif type(res) is _Sleep: # re-schedule coro after the given delay
            call_later(res.delay, resume)
        elif type(res) is _Block: # task has registered an fd for polling, the event loop needs to know which fd goes to which task
            _blocked_callbacks.append((res.fd, res.forRead, resume))
        else:
            assert False, f"Don't know what to do with yielded value {res!r}"
    
    call_soon(resume)

@types.coroutine
def sleep(duration):
    "Resume the running task after a delay."

    if duration <= 0:
        yield
    else:
        yield _Sleep(duration)

def adopt_socket(sock):
    "Register the given socket with the event loop."
    
    sock.setblocking(False)
    _poll.register(sock.fileno())
    _set_poll(sock.fileno(), False, False)

def disown_socket(sock):
    "Remove the given socket from the event loop."

    _poll.unregister(sock.fileno())

async def sock_accept(sock):
    while True: # in a loop in case the task is woken spuriously
        try:
            client = sock.accept()[0]
            adopt_socket(client)
            return client
        except BlockingIOError:
            await _wait_readable(sock.fileno())

async def sock_recv(sock, numBytes, flags = 0):
    while True:
        try:
            return sock.recv(numBytes, flags)
        except BlockingIOError:
            if flags != 0:
                raise
            await _wait_readable(sock.fileno())

async def sock_send(sock, bytesLike, flags = 0):
    while True:
        try:
            return sock.send(bytesLike, flags)
        except BlockingIOError:
            if flags != 0:
                raise
            await _wait_writable(sock.fileno())

# ----------------   Example app   ----------------

async def acceptor():
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_SEQPACKET)
    adopt_socket(sock)
    sock.bind("\0ree")
    sock.listen(16)
    
    while True:
        spawn_task(client_task(await sock_accept(sock)))

async def client_task(sock):
    for _ in range(25):
        await sock_send(sock, time.ctime().encode())
        await sleep(0.25)
    
    await sock_send(sock, b"bye")
    sock.shutdown(socket.SHUT_RDWR)
    disown_socket(sock)
    sock.close()

def main():
    spawn_task(acceptor())
    run_loop(True)

if __name__ == "__main__":
    main()
