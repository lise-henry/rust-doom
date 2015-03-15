use std::mem;
use std::slice;
use std::io::Read;
use math::{Vec2, Vec2f};
use super::types::{WadCoord, WadInfo, WadName, ChildId, WadNameCast};
use super::base::read_at_least;

#[derive(Copy)]
pub enum WadType { Initial, Patch }


const IWAD_HEADER: &'static [u8] = b"IWAD";
const PWAD_HEADER: &'static [u8] = b"PWAD";


pub fn wad_type_from_info(wad_info: &WadInfo) -> Option<WadType> {
    let id = &wad_info.identifier;
    if id == IWAD_HEADER {
        Some(WadType::Initial)
    } else if id == PWAD_HEADER {
        Some(WadType::Patch)
    } else {
        None
    }
}


pub fn read_binary<T: Copy, R: Read>(reader: &mut R) -> T {
    let mut loaded: T = unsafe { mem::zeroed() };
    let size = mem::size_of::<T>();
    unsafe {
        read_at_least(reader, slice::from_raw_parts_mut(
                (&mut loaded as *mut _ as *mut u8), size)).unwrap();
    }
    loaded
}


pub fn is_untextured(name: &WadName) -> bool {
    let bytes = name.as_bytes();
    bytes[0] == b'-' && bytes[1] == 0
}


pub fn is_sky_flat(name: &WadName) -> bool { name == &b"F_SKY1".to_wad_name() }


pub fn from_wad_height(x: WadCoord) -> f32 { (x as f32) / 100.0 }


pub fn from_wad_coords(x: WadCoord, y: WadCoord) -> Vec2f {
    Vec2::new(-from_wad_height(x), from_wad_height(y))
}


pub fn parse_child_id(id: ChildId) -> (usize, bool) {
    ((id & 0x7fff) as usize, id & 0x8000 != 0)
}
