class AsyncWebSocket
{
    constructor()
    {
        this.socket = null;
        this.resolvers = [];
    }

    connect(remote)
    {
        this.socket = new WebSocket(remote);

        return new Promise(
            (resolve, reject) =>
            {
                this.socket.addEventListener("open", () => resolve(this));
                this.socket.addEventListener("error", (err) => reject(err));
                this.socket.addEventListener("message", this._dispatchMsg.bind(this));
            }
        )
    }

    _dispatchMsg(event)
    {
        for (const resolve of this.resolvers)
            resolve(event.data);

        this.resolvers.splice(0, this.resolvers.length);
    }

    close(code = 1000, reason = "")
    {
        return new Promise(
            (resolve, reject) =>
            {
                if(this.socket == null || this.socket.readyState != 1)
                {
                    reject("Socket is already closed");

                    return;
                }

                resolve(this.socket.close(code, reason));

                this.socket = null;
            }
        );
    }

    send(msg)
    {
        return new Promise(
            (resolve, reject) =>
            {
                if (this.socket.readyState != 1)
                {
                    reject("Socket is not connected");

                    return;
                }

                resolve(this.socket.send(msg));
            }
        );
    }

    recv(timeout = 0)
    {
        let resolver = null;
        const promise = new Promise(
            (resolve, reject) =>
            {
                if(this.socket.readyState != 1)
                {
                    reject("Socket is not connected");

                    return;
                }

                resolver = resolve;

                if(timeout > 0)
                    setTimeout(
                        () => reject(`Socket recv timed out after ${timeout / 1000} seconds`),
                        timeout
                    );
            }
        );

        if(resolver) this.resolvers.push(resolver);

        return promise;
    }
}