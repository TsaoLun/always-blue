// deno_deploy.ts - Server for Deno Deploy
// Customized for Dioxus 0.6 projects

// Define static content type mapping
const contentTypeMap: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".wasm": "application/wasm",
  ".br": "", // Brotli compressed files, let the server use the content type of original extension
  ".gz": "", // Gzip compressed files, let the server use the content type of original extension
  ".ttf": "font/ttf",
  ".otf": "font/otf",
  ".woff": "font/woff",
  ".woff2": "font/woff2",
};

// Get content type based on file path
function getContentType(path: string): string {
  const ext = path.substring(path.lastIndexOf(".")).toLowerCase();
  return contentTypeMap[ext] || "text/plain; charset=utf-8";
}

// Load file based on path, supporting compressed versions
async function loadFile(path: string, acceptsBrotli = false): Promise<{ file: Uint8Array | null; compression?: string }> {
  // Debug log to see what path is being requested
  console.log(`Attempting to load file: ${path}`);
  
  // Clean up path - remove any "/./", normalize slashes
  const cleanPath = path.replace(/\/\.\//g, "/").replace(/\/+/g, "/");
  
  // Determine the base directory - when running from project root, use deploy/ subdirectory
  const baseDir = "./deploy/";
  // Remove leading ./ from cleanPath to avoid double dot
  const normalizedPath = cleanPath.startsWith("./") ? cleanPath.slice(2) : cleanPath;
  // Ensure the path starts with / for proper joining
  const finalPath = normalizedPath.startsWith("/") ? normalizedPath : `/${normalizedPath}`;
  const fullPath = `${baseDir}${finalPath}`;
  
  console.log(`Resolved file path: ${fullPath}`);
  
  // If client supports Brotli compression, try loading the .br file first
  if (acceptsBrotli) {
    try {
      const brotliFile = await Deno.readFile(`${fullPath}.br`);
      if (brotliFile) {
        console.log(`Successfully loaded Brotli file: ${fullPath}.br`);
        return { file: brotliFile, compression: "br" };
      }
    } catch (_) {
      // Ignore, continue to try other formats
    }
  }
  
  // Try to load the original file
  try {
    const file = await Deno.readFile(fullPath);
    console.log(`Successfully loaded file: ${fullPath}`);
    return { file };
  } catch (err: unknown) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    console.log(`Failed to load file ${fullPath}: ${errorMessage}`);
    return { file: null };
  }
}

// Main request handler function
async function handleRequest(req: Request): Promise<Response> {
  const url = new URL(req.url);
  let path = url.pathname;
  
  console.log(`Request: ${path}`);
  
  // If requesting root directory or path without extension and not an API path, serve index.html
  if (path === "/" || (!path.includes(".") && !path.startsWith("/api/"))) {
    path = "/index.html";
  }
  
  // Clean up path - normalize "/./assets/" to "/assets/" etc.
  path = path.replace(/\/\.\//g, "/");
  
  // Convert path to relative server path (remove leading slash)
  const filePath = `.${path}`;
  
  // Check if client supports Brotli compression
  const acceptsBrotli = req.headers.get("accept-encoding")?.includes("br") || false;
  
  // Set basic response headers
  const baseHeaders: Record<string, string> = {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "SAMEORIGIN",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
    "Referrer-Policy": "strict-origin-when-cross-origin",
  };
  
  // Handle OPTIONS requests (CORS preflight)
  if (req.method === "OPTIONS") {
    return new Response(null, {
      status: 204,
      headers: baseHeaders,
    });
  }
  
  // Try to load the file (original or compressed version)
  const { file, compression } = await loadFile(filePath, acceptsBrotli);
  
  if (file) {
    // Get content type; if .br extension, use the original extension's content type
    const contentType = getContentType(filePath);
    
    const headers: Record<string, string> = {
      ...baseHeaders,
      "Content-Type": contentType,
      "Cache-Control": path.includes("assets/") || path.includes("wasm/")
        ? "public, max-age=31536000, immutable" // Long-term cache for static assets
        : path.endsWith(".html")
          ? "public, max-age=3600" // Medium cache for HTML files
          : "public, max-age=86400", // One day cache for other files
    };
    
    // If returning a compressed file, add the appropriate encoding header
    if (compression === "br") {
      headers["Content-Encoding"] = "br";
    } else if (compression === "gzip") {
      headers["Content-Encoding"] = "gzip";
    }
    
    return new Response(file, { headers });
  }
  
  // If file not found and not an API request, try to serve index.html (SPA fallback)
  if (!path.startsWith("/api/") && path !== "/index.html") {
    console.log(`File not found, fallback to index.html: ${path}`);
    const { file: indexFile } = await loadFile("./index.html", acceptsBrotli);
    if (indexFile) {
      const headers: Record<string, string> = {
        ...baseHeaders,
        "Content-Type": "text/html; charset=utf-8",
        "Cache-Control": "max-age=3600",
      };
      return new Response(indexFile, { headers });
    }
  }
  
  // If nothing found, return 404
  console.log(`404: ${path}`);
  const { file: notFoundFile } = await loadFile("./404.html", acceptsBrotli);
  if (notFoundFile) {
    // If custom 404 page exists, use it
    return new Response(notFoundFile, {
      status: 404,
      headers: { 
        ...baseHeaders,
        "Content-Type": "text/html; charset=utf-8" 
      },
    });
  }
  
  // No custom 404 page, return plain text
  return new Response("404 Not Found", {
    status: 404,
    headers: { 
      ...baseHeaders,
      "Content-Type": "text/plain; charset=utf-8" 
    },
  });
}

// Start the server using the new Deno.serve API
Deno.serve(handleRequest);

console.log("Deno server running. Waiting for requests...");