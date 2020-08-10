const queue = [];
onmessage = (message) => {
    import(message.data).then(() => {
        for (const msg of queue) {
            onmessage(msg);
        }
    });
    onmessage = (message) => queue.push(message);
}