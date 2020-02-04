#include <emscripten.h>

void WasmLog(const char* msg, unsigned len);

EMSCRIPTEN_KEEPALIVE
void helloworld() {
    const char msg[] = "helloworld";
    WasmLog(msg, sizeof(msg) - 1);
}
