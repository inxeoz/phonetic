const http = require("http");
const fs = require("fs");
const path = require("path");
const net = require("net");

const TCP_HOST = "127.0.0.1";
const RUST_TCP_PORT = 3005; // Rust TCP Server Port
const SERVER_PORT = 3000;

// Create a single HTTP server for both file serving and API handling
const server = http.createServer((req, res) => {
    console.log("Server started with url: " + req.url);

    // Set CORS headers for API requests
    res.setHeader("Access-Control-Allow-Origin", "*");
    res.setHeader("Access-Control-Allow-Methods", "POST, OPTIONS");
    res.setHeader("Access-Control-Allow-Headers", "Content-Type");

    // Handle preflight requests
    if (req.method === "OPTIONS") {
        res.writeHead(204);
        res.end();
        return;
    }

    if (req.method === "POST" && req.url === "/convert") {
        let body = "";
        req.on("data", (chunk) => (body += chunk));
        req.on("end", () => {
            const { text } = JSON.parse(body);
            sendToTcpServer(text, (err, response) => {
                if (err) {
                    res.writeHead(500, { "Content-Type": "application/json" });
                    res.end(JSON.stringify({ error: err }));
                } else {
                    res.writeHead(200, { "Content-Type": "application/json" });
                    res.end(response);
                }
            });
        });
    } else {
        // Serve HTML & CSS files
        let filePath = req.url === "/" ? "index.html" : req.url;
        filePath = path.join(__dirname, filePath);

        let contentType = "text/html";
        if (filePath.endsWith(".css")) contentType = "text/css";

        fs.readFile(filePath, (err, content) => {
            if (err) {
                res.writeHead(404, { "Content-Type": "text/plain" });
                res.end("File Not Found");
            } else {
                res.writeHead(200, { "Content-Type": contentType });
                res.end(content);
            }
        });
    }
});

// TCP connection to Rust server
const sendToTcpServer = (text, callback) => {
    const client = new net.Socket();

    client.connect(RUST_TCP_PORT, TCP_HOST, () => {
        client.write(JSON.stringify({ text }));
    });

    client.on("data", (data) => {
        callback(null, data.toString());
        client.destroy();
    });

    client.on("error", (err) => callback(err.message));
};

server.listen(SERVER_PORT, () => console.log(`Server running at http://127.0.0.1:${SERVER_PORT}`));