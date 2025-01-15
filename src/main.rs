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

static mut wpoint: i32 = 0;
static mut windows: Vec<xlib::Window> = Vec::new();

const panel_hei: u32 = 50;

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

		let panel = createPanel(disp, _root);

		let mut flag = true;

		let mut e: xlib::XEvent = MaybeUninit::uninit().assume_init();
		
		loop {
			xlib::XNextEvent(disp, &mut e as *mut xlib::XEvent);
			if flag && e.type_ == 19 {
				flag = false;
				continue;
			}
			match e.type_ {
				2 => {
					//println!("Key: { }", e.key.keycode);
					
					match e.key.keycode {
						11 => {
							if wpoint + 1 <= windows.len() as i32 {	
								wpoint += 1;
							}

							showWindow(disp);
						}
						
						10 => {
							if wpoint - 1 >= 1 {
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
					println!("Code: { }", e.type_);
				}
			}
			
			updatePanel(disp, _root, panel);
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

                xlib::XMoveResizeWindow(disp, ev.window, 0, (panel_hei+4).try_into().unwrap(), wid-1, hei-1);

                xlib::XMapWindow(disp, ev.window);
        }
}

fn unmapNotifyFunc(disp: *mut Display, e: xlib::XEvent) {
	unsafe {
		let ev = e.map_request;

		if wpoint >= 1 && wpoint <= windows.len() as i32 {
			//let check = |x: &mut xlib::Window| *x == windows[(wpoint-1) as usize];
			println!("wp {} ws {}", wpoint-1, windows.len());
			//windows.pop_if(check);
			//for
			windows.remove((wpoint-1) as usize); 
			println!("wp {} ws {}", wpoint-1, windows.len());
		}
			
		if wpoint - 1 >= 1 {
			wpoint -= 1;
			showWindow(disp);
		} else {
			if (windows.len() as i32) == 0 {
				wpoint = 0;
			} else {
				wpoint = 1;
			}
		}
	}
}

fn showWindow(disp: *mut Display) {
	unsafe {
		if wpoint >= 1 && wpoint < windows.len() as i32 {
			let wnd = windows[(wpoint-1) as usize];
			
			let screen = xlib::XDefaultScreen(disp);

                	let wid: u32 = xlib::XDisplayWidth(disp, screen).try_into().unwrap();
                	let hei: u32 = xlib::XDisplayHeight(disp, screen).try_into().unwrap();

                	xlib::XRaiseWindow(disp, wnd);
		}
	}
}

fn createWindow(disp: *mut Display, root_: c_ulong, w: i32, h: i32, x: i32, y: i32) -> xlib::Window {
	unsafe {
		let screen = xlib::XDefaultScreen(disp);
		let wnd = xlib::XCreateSimpleWindow(disp, root_, x, y, w.try_into().unwrap(), h.try_into().unwrap(), 2, xlib::XWhitePixel(disp, screen), xlib::XBlackPixel(disp, screen));

		xlib::XMapWindow(disp, wnd);
		xlib::XRaiseWindow(disp, wnd);
		return wnd;
	}
}

fn createPanel(disp: *mut Display, root_: c_ulong) -> xlib::Window {
	unsafe {
		let screen = xlib::XDefaultScreen(disp);
		let wid: i32 = xlib::XDisplayWidth(disp, screen).try_into().unwrap();
		let wnd = createWindow(disp, root_, 1, 1, 0, 0);

		xlib::XMoveResizeWindow(disp, wnd, 0, 0, (wid-4).try_into().unwrap(), panel_hei);
		return wnd;
	}
}

fn updatePanel(disp: *mut Display, root_: c_ulong, wnd: xlib::Window) {
	unsafe {
		let screen = xlib::XDefaultScreen(disp);
		let wid: i32 = xlib::XDisplayWidth(disp, screen).try_into().unwrap();
		let black = xlib::XBlackPixel(disp, screen);
		let white = xlib::XWhitePixel(disp, screen);
		let gc = xlib::XCreateGC(disp, wnd, 0, std::ptr::null_mut());
		let halfwid: i32 = (wid/2).try_into().unwrap();

		xlib::XSetBackground(disp, gc, black);
		xlib::XSetForeground(disp, gc, black);
		xlib::XFillRectangle(disp, wnd, gc, 0, 0, wid.try_into().unwrap(), 50);
                xlib::XSetForeground(disp, gc, white);
		xlib::XDrawString(disp, wnd, gc, halfwid-5, 25, (wpoint.to_string()+"\0").as_ptr() as *const i8, 1);
	}
}
