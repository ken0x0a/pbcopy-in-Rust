extern crate clap;
#[macro_use]
extern crate cfg_if;
use clap::{App};
use std::io::{self, Read};

cfg_if! {
    if #[cfg(target_os="macos")] {
        extern crate objc_foundation;
        #[macro_use]
        extern crate objc;
        extern crate objc_id;
        use objc::runtime::{Object, Class};
        use objc_id::{Id};
        use objc_foundation::{NSArray, NSString};
        use objc_foundation::{INSArray, INSString};
        // Required to bring NSPasteboard into the path of the class-resolver
        #[link(name = "AppKit", kind = "framework")]
        extern "C" {} // opens up the search space to C
    }
}

// #[cfg(target_os="linux")]
// impl From<x11_clipboard::error::Error> for io::Error {
//     fn from(err: x11_clipboard::error::Error) -> Self {
//         match err {
//             Set(atom) => {

//             }
//             XcbConn(connError) => {

//             }
//         }
//     }
// }

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

cfg_if! {
    if #[cfg(target_os="linux")] {
        extern crate x11_clipboard;
        use std::time::Duration;
        use std::marker::PhantomData;
        use x11_clipboard::Atoms;
        use x11_clipboard::Clipboard as X11Clipboard;
        use x11_clipboard::xcb::xproto::Atom;
    }
}


#[cfg(target_os="linux")]
fn atom(atoms: &Atoms) -> Atom {
    atoms.clipboard
}


#[cfg(target_os="linux")]
fn add_to_clipboard(data: &str) -> GenResult<()> {

    //    Whenever a client wants to transfer data to the clipboard:
    //        It should assert ownership of the CLIPBOARD.
    let clipboard = X11Clipboard::new(); // Error(XcbConn(ClosedParseErr)
    let clipboard = clipboard?;
    //        If it succeeds in acquiring ownership, it should be prepared to respond to a request for
    //              the contents of the CLIPBOARD in the usual way (retaining the data to be able to
    //              return it). The request may be generated by the clipboard client described below.
    //        If it fails to acquire ownership, a cutting client should not actually perform the cut or
    //              provide feedback that would suggest that it has actually transferred data to the clipboard.

    println!("1={}", data.to_string());
    let data1 = "test";

    let atom_1 = clipboard.setter.atoms.clipboard;
    let s = clipboard.setter.atoms.utf8_string;
    println!("almost success=");
    clipboard.store(atom_1, s, data1);

    println!("succes");
    return Ok(());

}

#[cfg(target_os="macos")]
fn add_to_clipboard(data: &str) -> GenResult<()> {
    let cls = Class::get("NSPasteboard").ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Class::get(NSPasteboard)"))?;
    let clipboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
    let clipboard: Id<Object> = unsafe { Id::from_ptr(clipboard) };
    let string_array = NSArray::from_vec(vec![NSString::from_str(data)]);
    let _: usize = unsafe { msg_send![clipboard, clearContents] };
    let success: bool = unsafe { msg_send![clipboard, writeObjects:string_array] };

    println!("success={}", success);
    return Ok(());
}

fn main() {
    println!("hiiii");

    App::new("Rpbcopy")
        .version("0.1.0")
        .author("Bradley and Liuda")
        .about("pbcopy clone written in Rust")
        .get_matches();
    let mut buffer = String::new();
    println!("hiiii");
    println!("hiiii buff={}", &buffer);

    io::stdin().read_to_string(&mut buffer).expect("failed to read buffer");
    println!("hiiii buff={}", &buffer);

    add_to_clipboard(&buffer).unwrap();
}