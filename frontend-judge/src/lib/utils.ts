// place files you want to import through the `$lib` alias in this folder.

const readStream = async (stream: ReadableStream<Uint8Array>) => {
    const reader = stream.getReader();
    let chunks = '';

    while (true) {
        const { value, done } = await reader.read();
        if (done) {
            break;
        }
        chunks += new TextDecoder().decode(value);
    }
    return chunks;
}


export { readStream };