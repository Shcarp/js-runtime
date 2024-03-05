({ fetch }) => {
    globalThis.fetch = async (url) => {
        return await fetch(url);
    };
}