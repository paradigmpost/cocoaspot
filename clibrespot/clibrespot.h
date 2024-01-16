#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ViewModel ViewModel;

struct ViewModel *spot_init_view_model(void);

void spot_login(struct ViewModel *view_model, const char *user, const char *pass);

void spot_play(struct ViewModel *view_model, const char *track_id);

void spot_resume(struct ViewModel *view_model);

void spot_pause(struct ViewModel *view_model);

void spot_listen_for_events(struct ViewModel *view_model,
                            void *context,
                            void (*is_track_loaded_cb)(bool, void*),
                            void (*is_playing_cb)(bool, void*));
