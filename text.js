fetch("http://localhost:3000", { method: "GET" }).then((response) =>
    response.text().then((text) => console.log(text))
);
