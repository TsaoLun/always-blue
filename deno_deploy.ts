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
    
    return new Response(file, {
      headers: {
        "Content-Type": mimeType,
        "Cache-Control": pathname.includes("/pkg/") 
          ? "public, max-age=31536000, immutable" 
          : "public, max-age=3600",
        "Access-Control-Allow-Origin": "*",
      },
    });
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