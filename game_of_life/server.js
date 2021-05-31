
/*
    NOT FOR PRODUCTION!

    This is a simple  nodejs server for local testing though I am
    attempting to make it more robust as time goes on.

    Right now, it only handles static files with no caching.
*/

const http = require("http");
const fs = require("fs");
const { stringify } = require("querystring");

const PORT = 8080;

const allowed_file_types = {
    "html": { content_type: "text/html" },
    "js": { content_type: "application/javascript" },
    "css": { content_type: "text/css" },
    "map": { content_type: "text/plain" },
    "wasm": { content_type: "application/wasm" }
};

const content_encoding = "charset=UTF-8";

const regx_double_periods = /\.\./;
const regx_path_valid_chars = /^[a-zA-Z0-9-_/.]+$/;

const validate_path_chars = function(path) {
    // I don't want to accept paths with .. because I don't
    // want to accidentally support relative paths
    if (!regx_double_periods.test(path)) {
        // I only want to accept a limited set of charcters to
        // minimize attack vectors
        if (regx_path_valid_chars.test(path)) {
            return true;
        }
    }
    return false;
};

const concat_path = function(/* variable args */) { 
    let result_path = "";
    if (arguments.length == 0) {
        return result_path;
    }
    result_path = arguments[0];
    for(let i = 1; i < arguments.length; i++) {
        const part = arguments[i];
        if (part[0] == "/") {
            result_path += part;
        }
        else {
            result_path += "/" + part;
        }
    }
    return result_path;
};

const validate_local_path = function (path, request) {
    const file_extension = path.substring(path.lastIndexOf(".")+1).toLowerCase();
    if (file_extension && file_extension.length > 0) {
        if (allowed_file_types.hasOwnProperty(file_extension)) {
            const local_path = concat_path(__dirname, "www", path);
            console.log(local_path);
            if (fs.existsSync(local_path)) {
                request.local_path = local_path;
                request.file_extension = file_extension;
                return true;
            }
        }
    }
    return false;
};

const wrap_request = function(request) {
    let url = request.url;
    // incomming url checks - anything invalid changes to default
    if (typeof url !== "string" || 
        url.length === 0 || 
        url === "/" || 
        !url.startsWith("/")) {
        url = "/default.html";
    }
    const state = { wrapped_request: request, url: url };

    // split path from query string
    const i = url.indexOf("?");
    if (i > 0) {
        state.path = url.substr(0, i);
        state.query_string = url.substr(i+1);
    } else {
        state.path = url;
    }

    state.method = request.method.toUpperCase();
    
    return state;
};

const get_content_type = function(ext) { 
    if (ext == "wasm") {
        return allowed_file_types[ext].content_type;
    }
    else {
        return allowed_file_types[ext].content_type + "; " + content_encoding;
    }
};

const handle_static_file = function(request, response) {
    fs.readFile(request.local_path, function(error, data) {
        if (error) {
            handle_error(request, response, 500, "error reading file.");
        } else {
            let headers = { "Cache-Control": "no-cache" };
            headers["Content-Type"] = get_content_type(request.file_extension);    
            response.writeHead(200, headers);
            response.end(data);
            console.log(request.method + " 200 " + request.url);
        }
    });
};

const handle_error = function(request, response, status, error) {
    let headers = { "Cache-Control": "no-cache", "Content-Type": "text/plain; charset=utf-8" };
    response.writeHead(status, headers);
    if (error) {
        response.write(error);
    }
    response.end();
    console.log(request.method + " " + status + " " + request.url);
};

const handle_request = function(request, response) {
    request = wrap_request(request);

    if (request.method !== "GET") {
        handle_error(request, response, 405, "only GET is allowed at this time.");
    }

    // if the path has invalid chars, refuse to process any further
    else if (validate_path_chars(request.path)) {
        if (validate_local_path(request.path, request)) {
            handle_static_file(request, response);
        } else {
            // maybe we allow some kind of api call in the future
            handle_error(request, response, 404);
        }
    } 
    
    else {
        handle_error(request, response, 400, "invalid request path.");
    }
};

const server = http.createServer(handle_request);

server.listen(PORT, function(){
    console.log("Server Listening at http://localhost:" + PORT);
});