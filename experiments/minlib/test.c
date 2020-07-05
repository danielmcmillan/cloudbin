#include <stdlib.h>
#include <string.h>
#include <emscripten/emscripten.h>

extern void print(char *ptr, size_t length);

char *EMSCRIPTEN_KEEPALIVE setup()
{
  char *buf = malloc(40 * 25);
  strcpy(buf, "Hello world\n");
  print(buf, 12);
  buf[120] = 32;
  return buf;
}

void EMSCRIPTEN_KEEPALIVE destroy(char *buf)
{
  free(buf);
}

char *EMSCRIPTEN_KEEPALIVE draw(char *buf)
{
  strcpy(buf + 12, "\nFrom draw function");
  buf[120] = buf[120] >= 127 ? 32 : buf[120] + 1;
  return buf;
}
