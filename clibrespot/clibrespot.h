#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Instance Instance;

Runtime *spot_init_runtime(void);

struct Instance *spot_init_player(Runtime *runtime, const char *user, const char *pass);

void spot_play(Runtime *runtime, struct Instance *instance, const char *track_id);

void spot_resume(struct Instance *instance);

void spot_pause(struct Instance *instance);
