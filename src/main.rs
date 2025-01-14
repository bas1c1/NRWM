#![feature(vec_pop_if)]
#[macro_use]
extern crate lazy_static;
extern crate x11;

use x11::xlib;
use x11::keysym;
use x11::xlib::{Display};
use std::mem::MaybeUninit;
use std::mem;
use std::os::raw::{c_char, c_ulong, c_uchar};

const cevents: i64 = xlib::SubstructureNotifyMask | xlib::StructureNotifyMask | xlib::KeyPressMask | xlib::KeyReleaseMask |
		     xlib::ButtonPressMask | xlib::ButtonReleaseMask;

static mut wpoint: i32 = -1;
static mut windows: Vec<xlib::Window> = Vec::new();

fn main() {
	let disp = unsafe { xlib::XOpenDisplay(std::ptr::null()) };

	if disp.is_null() {
		panic!("not working((");
	}

	let _root = unsafe { xlib::XDefaultRootWindow(disp) };

	println!("NRWM started");

	unsafe {
		let mut wattr: xlib::XSetWindowAttributes = MaybeUninit::zeroed().assume_init();
		wattr.event_mask = cevents;

		xlib::XChangeWindowAttributes(disp, _root, xlib::CWEventMask, &mut wattr);

		let mod_super = xlib::ControlMask;

		grabKey(disp, _root, xlib::XKeysymToKeycode(disp, keysym::XK_1.into()), mod_super);
		grabKey(disp, _root, xlib::XKeysymToKeycode(disp, keysym::XK_2.into()), mod_super);		

		let cursor = xlib::XCreateFontCursor(disp, 68);

		xlib::XDefineCursor(disp, _root, cursor);
	
		xlib::XSync(disp, 0);

		let mut e: xlib::XEvent = MaybeUninit::uninit().assume_init();
		
		loop {
			xlib::XNextEvent(disp, &mut e as *mut xlib::XEvent);

			match e.type_ {
				2 => {
					//println!("Key: { }", e.key.keycode);
					
					match e.key.keycode {
						11 => {
							if wpoint + 1 < windows.len().try_into().unwrap() {	
								wpoint += 1;
							}

							showWindow(disp);
						}
						
						10 => {
							if wpoint - 1 >= 0 {
								wpoint -= 1;
							}

							showWindow(disp);
						}
						_always => {}
					}
				}

				19 => {
					mapNotifyFunc(disp, e);
				}

				18 => {
					unmapNotifyFunc(disp, e);
				}
	
				_always=> {
					println!("Unexpected: { }", e.type_);
				}
			}
		}

		xlib::XCloseDisplay(disp);
	};
}

fn grabKey(disp: *mut Display, root_: c_ulong, key: c_uchar, modd: u32) {
	unsafe {
		//let sym = xlib::XStringToKeysym(key);
		//let code = xlib::XKeysymToKeycode(disp, sym);
		xlib::XGrabKey(disp, key.into(), modd, root_, 0, 1, 1); 
	}
}

fn mapNotifyFunc(disp: *mut Display, e: xlib::XEvent) {
	unsafe {
                let ev = e.map_request;

		windows.push(ev.window);

		wpoint += 1;		

		let screen = xlib::XDefaultScreen(disp);
		
		let wid: u32 = xlib::XDisplayWidth(disp, screen).try_into().unwrap();
		let hei: u32 = xlib::XDisplayHeight(disp, screen).try_into().unwrap();		

                xlib::XRaiseWindow(disp, ev.window);
                //xlib::XSetInputFocus(disp, ev.window, xlib::RevertToPointerRoot, xlib::CurrentTime);
                xlib::XMoveResizeWindow(disp, ev.window, 0, 0, wid-1, hei-1);

                xlib::XMapWindow(disp, ev.window);
        }
}

fn unmapNotifyFunc(disp: *mut Display, e: xlib::XEvent) {
	unsafe {
		let ev = e.map_request;
		let check = |x: &mut xlib::Window| *x == windows[wpoint as usize];
		
		windows.pop_if(check);
		
		if wpoint - 1 >= 0 {
			wpoint -= 1;
			showWindow(disp);
		} else {
			wpoint = 0;
		}
	}
}

fn showWindow(disp: *mut Display) {
	unsafe {
		println!("WPOINT {}", wpoint);
		let wnd = windows[wpoint as usize];

		let screen = xlib::XDefaultScreen(disp);

                let wid: u32 = xlib::XDisplayWidth(disp, screen).try_into().unwrap();
                let hei: u32 = xlib::XDisplayHeight(disp, screen).try_into().unwrap();

                xlib::XRaiseWindow(disp, wnd);
                //xlib::XSetInputFocus(disp, ev.window, xlib::RevertToPointerRoot, xlib::CurrentTime);
                xlib::XMoveResizeWindow(disp, wnd, 0, 0, wid-1, hei-1);
	}
}
