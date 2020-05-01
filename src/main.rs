#![no_std]
#![no_main]
#![feature(try_trait)]
#![feature(abi_efiapi)]
extern crate alloc;
use uefi::prelude::*;
use log::*;
use uefi::proto::console::gop::{GraphicsOutput, FrameBuffer, PixelFormat, ModeInfo, BltPixel, BltOp, BltRegion};
use core::ops::{IndexMut, Index};
use alloc::vec::Vec;
use alloc::vec;
use core::borrow::Borrow;

type Vec2 = (usize, usize);

struct Surface<'g> {
    gop: &'g mut GraphicsOutput<'g>,
    buf: Vec<BltPixel>,
    mi: ModeInfo,
}

impl<'g> Surface<'g>{
    pub fn create(gop: &'g mut GraphicsOutput<'g>) -> Surface<'g> {
        let mi = gop.current_mode_info();
        Surface {
            buf: vec!(BltPixel::from(0); mi.stride() * mi.resolution().1),
            gop,
            mi,
        }
    }
    pub fn clear(&mut self) {
        self.buf = vec!(BltPixel::from(0); self.mi.stride() * self.mi.resolution().1);
    }
    pub fn draw_square(&mut self, pos: Vec2, size: Vec2, pixel: BltPixel) {
        let stride = self.mi.stride();
        for x in pos.0..pos.0 + size.0 {
            for y in pos.1..pos.1 + size.1 {
                self.buf[stride * y + x] = pixel;
            }
        }
    }
    pub fn blt(&mut self) {
        self.gop.blt(BltOp::BufferToVideo {
            buffer: &self.buf[..],
            src: BltRegion::SubRectangle {
                coords: (0,0),
                px_stride: self.mi.stride(),
            },
            dims: self.mi.resolution(),
            dest: (0,0),
        }).unwrap_success();
    }
    pub fn dims(&self) -> Vec2 {
        self.mi.resolution()
    }
}

#[entry]
fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {

    uefi_services::init(&system_table).expect_success("Failed to initialize utilities");

    system_table.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");

    let services = system_table.boot_services();

    let gop_proto = unsafe { services.locate_protocol::<GraphicsOutput>().unwrap_success().get().as_mut().unwrap() };

    let mode_info = gop_proto.current_mode_info();

     log!(Level::Info, "GOP Mode: {:?}", mode_info);

    let mut surface = Surface::create(gop_proto);

    let mut x = (surface.dims().0/2,surface.dims().1/2);
    let mut dx  = (1isize,1isize);

    loop {
        surface.clear();
        surface.draw_square((x.0-10,x.1-10), (20,20), BltPixel::new(0,255,0));
        surface.blt();

        if x.0 <= 10 || x.0 >= surface.dims().0 - 10 {
            dx.0 = -dx.0;
        }

        if x.1 <= 10 || x.1 >= surface.dims().1 - 10 {
            dx.1 = -dx.1;
        }

        x.0 = (x.0 as isize + dx.0) as usize;
        x.1 = (x.1 as isize + dx.0) as usize;

    }

    Status::SUCCESS
    
}
