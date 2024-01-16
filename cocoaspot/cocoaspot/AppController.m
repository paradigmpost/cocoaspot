//
//  AppController.m
//  cocoaspot
//
//  Created by Travis T. Destiny on 1/12/24.
//

#import "AppController.h"
#import <stdlib.h>
#import <dispatch/dispatch.h>

#import "clibrespot.h"

@interface AppController (Private)

- (void)isTrackLoadedChanged:(BOOL)is_track_loaded;
- (void)isPlayingChanged:(BOOL)is_playing;

@end

void is_track_loaded_listener(bool is_track_loaded, void *context) {
    AppController *controller = (__bridge AppController *)(context);
    
    dispatch_async(dispatch_get_main_queue(), ^{
        if (controller != nil) {
            [controller isTrackLoadedChanged:is_track_loaded == true ? YES : NO];
        }
    });
}

void is_playing_listener(bool is_playing, void *context) {
    AppController *controller = (__bridge AppController *)(context);
    
    dispatch_async(dispatch_get_main_queue(), ^{
        if (controller != nil) {
            [controller isPlayingChanged:is_playing == true ? YES : NO];
        }
    });
}

@implementation AppController {
    dispatch_queue_t player_queue;
    ViewModel *view_model;
    BOOL is_track_loaded;
    BOOL is_playing;
}

- (void)isTrackLoadedChanged:(BOOL)is_track_loaded {
    NSLog(@"isTrackLoadedChanged! %@", is_track_loaded ? @"Yes" : @"No");

    self->is_track_loaded = is_track_loaded;
    
    if (is_track_loaded == NO) {
        self->is_playing = NO;
    }
    
    [self updateToggleButton];
}

- (void)isPlayingChanged:(BOOL)is_playing {
    NSLog(@"isPlayingChanged! %@", is_playing ? @"Yes" : @"No");

    self->is_playing = is_playing;
    
    if (is_playing == YES) {
        self->is_track_loaded = YES;
    }
    
    [self updateToggleButton];
}

- (void)updateToggleButton {
    if (self->is_track_loaded == NO) {
        [self toggleButton].title = @"Play";
    } else if (self->is_playing == NO) {
        [self toggleButton].title = @"Resume";
    } else {
        [self toggleButton].title = @"Pause";
    }
}

- (id)init {
    self = [super init];
    if (self) {
        player_queue = dispatch_queue_create("player", &_dispatch_queue_attr_concurrent);
        self->is_track_loaded = NO;
        self->is_playing = NO;
        
        dispatch_sync(player_queue, ^{
            ViewModel *view_model = spot_init_view_model();

            spot_listen_for_events(
                view_model,
                (__bridge void *)(self),
                is_track_loaded_listener,
                is_playing_listener
            );
            
            dispatch_async(dispatch_get_main_queue(), ^{
                self->view_model = view_model;
            });
        });
    }
    return self;
}

- (IBAction)play:(NSButton *)sender {
    if (self->is_playing) {
        [self pause];
        return;
    }
    
    if (self->is_track_loaded) {
        [self resume];
        return;
    }
    
    char *user = malloc(100);
    [[self username].stringValue getCString:user maxLength:100 encoding:NSUTF8StringEncoding];
    
    char *pass = malloc(100);
    [[self password].stringValue getCString:pass maxLength:100 encoding:NSUTF8StringEncoding];
    
    char *trackId = malloc(100);
    [[self trackId].stringValue getCString:trackId maxLength:100 encoding:NSUTF8StringEncoding];

    
    dispatch_async(player_queue, ^{
        spot_login(self->view_model, user, pass);
        
        // TODO: necessary to free on the same thread?
        dispatch_sync(dispatch_get_main_queue(), ^{
            free(user);
            free(pass);
        });
        
//        if (!self->player) {
//            [self showError:@"Player failed to initialize, likely due to bad credentials. Try again."];
//            return;
//        }
        
        NSLog(@"pre: spot_play");
        
        spot_play(self->view_model, trackId);
        
        NSLog(@"test...");
        dispatch_sync(dispatch_get_main_queue(), ^{
            free(trackId);
        });
    });
}

- (void)pause {
    dispatch_async(player_queue, ^{
        spot_pause(self->view_model);
    });
}

- (void)resume {
    dispatch_async(player_queue, ^{
        spot_resume(self->view_model);
    });
}

#pragma mark - helpers

- (void)showError:(NSString *)message {
    NSAlert *alert = [[NSAlert alloc] init];
    [alert setMessageText:message];
    [alert addButtonWithTitle:@"Okay"];
    [alert runModal];
}

@end
