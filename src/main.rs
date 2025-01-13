#[macro_use]
extern crate lazy_static;
extern crate x11;

use x11::xlib;
use x11::xlib::{Display};
use std::mem::MaybeUninit;
use std::mem;

fn main() {
	let disp = unsafe { xlib::XOpenDisplay(std::ptr::null()) };

	if disp.is_null() {
		panic!("not working((");
	}

	let _root = unsafe { xlib::XDefaultRootWindow(disp) };

	println!("NRWM started");

	unsafe {
		let cursor = xlib::XCreateFontCursor(disp, 68);

		xlib::XDefineCursor(disp, _root, cursor);
	
		xlib::XSelectInput(disp, _root, xlib::SubstructureRedirectMask);
		xlib::XSync(disp, 0);

		let mut e: xlib::XEvent = MaybeUninit::uninit().assume_init();
		
		loop {
			xlib::XNextEvent(disp, &mut e as *mut xlib::XEvent);

			match e.type_ {
				20 => {
					mapRequestFunc(disp, e);
				}
	
				_always=> {
					println!("Unexpected event lol");
				}
			}

			xlib::XSync(disp, 0);
		}

		xlib::XCloseDisplay(disp);
	};
}

fn mapRequestFunc(disp: *mut Display, e: xlib::XEvent) {
	unsafe {
		let ev = e.map_request;

		xlib::XMoveResizeWindow(disp, ev.window, 0, 0, 700, 700);

		xlib::XMapWindow(disp, ev.window);
	}
}
