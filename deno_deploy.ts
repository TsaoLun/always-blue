// deno_deploy.ts - Server for Deno Deploy
// Simplified version for Slint WASM projects

const MIME_TYPES: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".mjs": "application/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".wasm": "application/wasm",
  ".ttf": "font/ttf",
  ".otf": "font/otf",
  ".woff": "font/woff",
  ".woff2": "font/woff2",
};

function getMimeType(pathname: string): string {
  const ext = pathname.substring(pathname.lastIndexOf(".")).toLowerCase();
  return MIME_TYPES[ext] || "text/plain; charset=utf-8";
}

async function serveFile(pathname: string): Promise<Response | null> {
  try {
    // Convert URL path to file path
    const filePath = pathname.startsWith("/") ? `.${pathname}` : `./${pathname}`;
    console.log(`Trying to serve file: ${filePath}`);
    
    const file = await Deno.readFile(filePath);
    const mimeType = getMimeType(pathname);
    
    console.log(`Successfully served: ${filePath} as ${mimeType}`);
    
    // 获取文件修改时间作为版本标识
    const stat = await Deno.stat(filePath);
    const lastModified = stat.mtime?.toUTCString() || new Date().toUTCString();
    const etag = `"${stat.size}-${stat.mtime?.getTime() || Date.now()}"`;
    
    const headers: Record<string, string> = {
      "Content-Type": mimeType,
      "Last-Modified": lastModified,
      "ETag": etag,
      "Access-Control-Allow-Origin": "*",
    };

    // 针对不同文件类型设置缓存策略
    if (pathname.includes("/pkg/") && pathname.endsWith(".wasm")) {
      // WASM 文件 - 短期缓存，便于开发时更新
      headers["Cache-Control"] = "public, max-age=300, must-revalidate";
    } else if (pathname.includes("/pkg/") && pathname.endsWith(".js")) {
      // WASM 生成的 JS 文件 - 短期缓存
      headers["Cache-Control"] = "public, max-age=300, must-revalidate";
    } else if (pathname.endsWith(".html")) {
      // HTML 文件 - 不缓存，总是检查更新
      headers["Cache-Control"] = "no-cache, no-store, must-revalidate";
      headers["Pragma"] = "no-cache";
      headers["Expires"] = "0";
    } else {
      // 其他静态资源 - 短期缓存
      headers["Cache-Control"] = "public, max-age=3600, must-revalidate";
    }
    
    return new Response(file, { headers });
  } catch (error) {
    console.log(`Failed to serve ${pathname}:`, (error as Error).message);
    return null;
  }
}

async function handleRequest(request: Request): Promise<Response> {
  const url = new URL(request.url);
  let pathname = url.pathname;
  
  console.log(`Request: ${pathname}`);
  
  // Handle root path
  if (pathname === "/") {
    pathname = "/index.html";
  }
  
  // Handle CORS preflight
  if (request.method === "OPTIONS") {
    return new Response(null, {
      status: 204,
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
        "Access-Control-Allow-Headers": "Content-Type",
      },
    });
  }
  
  // Try to serve the requested file
  const fileResponse = await serveFile(pathname);
  if (fileResponse) {
    return fileResponse;
  }
  
  // For missing static assets, return 404
  const isStaticAsset = /\.(js|css|wasm|png|jpg|jpeg|gif|svg|ico|json|txt|xml|woff|woff2|ttf|otf|d\.ts)$/i.test(pathname);
  if (isStaticAsset) {
    console.log(`Static asset not found: ${pathname}`);
    return new Response("File not found", {
      status: 404,
      headers: {
        "Content-Type": "text/plain",
        "Access-Control-Allow-Origin": "*",
      },
    });
  }
  
  // For non-static routes, fall back to index.html (SPA routing)
  console.log(`Fallback to index.html for: ${pathname}`);
  const indexResponse = await serveFile("/index.html");
  if (indexResponse) {
    return indexResponse;
  }
  
  // If index.html is also not found, return 404
  return new Response("Not Found", {
    status: 404,
    headers: {
      "Content-Type": "text/plain",
      "Access-Control-Allow-Origin": "*",
    },
  });
}

// Start the server
Deno.serve(handleRequest);

console.log("Deno server running. Access your app at http://localhost:8000");