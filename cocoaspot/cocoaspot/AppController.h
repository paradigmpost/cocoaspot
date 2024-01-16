//
//  AppController.h
//  cocoaspot
//
//  Created by Travis T. Destiny on 1/12/24.
//

#import <Cocoa/Cocoa.h>

NS_ASSUME_NONNULL_BEGIN

@interface AppController : NSObject

#pragma mark - No Player View

@property (weak) IBOutlet NSView *noPlayerView;
@property (weak) IBOutlet NSTextField *username;
@property (weak) IBOutlet NSTextField *password;

- (IBAction)logIn:(NSButton *)sender;

#pragma mark - Player View

@property (weak) IBOutlet NSView *playerView;
@property (weak) IBOutlet NSTextField *trackId;
@property (weak) IBOutlet NSButton *toggleButton;

- (IBAction)play:(NSButton *)sender;


@end

NS_ASSUME_NONNULL_END
