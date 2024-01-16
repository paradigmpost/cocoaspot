//
//  AppController.h
//  cocoaspot
//
//  Created by Travis T. Destiny on 1/12/24.
//

#import <Cocoa/Cocoa.h>

NS_ASSUME_NONNULL_BEGIN

@interface AppController : NSObject

@property (strong) IBOutlet NSTextField *username;
@property (strong) IBOutlet NSTextField *password;
@property (strong) IBOutlet NSTextField *trackId;
@property (strong) IBOutlet NSButton *toggleButton;

- (IBAction) play:(NSButton *)sender;

@end

NS_ASSUME_NONNULL_END
