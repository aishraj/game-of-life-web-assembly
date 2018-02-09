use std::mem;
use std::slice;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

fn set_pixel(pixels: &mut [u8], width: usize, x: usize, y: usize) {
    let offset = x*4 + y*4 * width;

    //Set color to #e56c27
    pixels[(offset+0)] = 229;
    pixels[(offset+1)] = 108;
    pixels[(offset+2)] = 39;
    pixels[(offset+3)] = 255;
}

fn count_neighbours(pixels: &[u8], width: usize, x: usize, y: usize) -> i32 {
    let x = x as i32;
    let y = y as i32;
    let width = width as i32;
    let xy_pairs = [(-1,-1),(-1,0), (-1,1), (0,1), (1,1), (1,0), (-1,1), (0,-1)];
    let c = xy_pairs.into_iter().fold(0, |cnt, &(x_off, y_off)| {
        let res = if is_filled(pixels, width, x + x_off, y + y_off) {
            cnt + 1
        } else {
            cnt
        };
        res
    });
    c
}

fn is_filled(pixels: &[u8], width: i32, x: i32, y: i32) -> bool {
    if x >= width || y >= width {
        return false;
    }
    let offset = x*4 + y*4 * width;
    let screen_size = pixels.len() as i32;
    if offset < 0 || offset > screen_size || offset % 4 != 0{
        return false;
    }
    let offset = offset as usize;
    for i in 0..4 {
        if pixels[(offset + i)] != 0 {
            return true;
        }
    }
    return false;
}

fn clear_cell(pixels: &mut [u8], width: usize, x: usize, y: usize) {
    let offset = x*4 + y*4 * width;
    for i in 0..4 {
        pixels[(offset + i)] = 0;
    }
}

#[no_mangle]
pub extern "C" fn clear(pointer: *mut u8, width: usize, height: usize) {
    let byte_size = width * height * 4;
    let buf = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    for i in buf.iter_mut() {
        *i = 0;
    }
}

#[no_mangle]
pub extern "C" fn next_generation(pointer: *mut u8, width: usize, height: usize) -> *mut c_void {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = width * height * 4;
    let buf = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let next_gen_ptr = alloc(byte_size);
    let next_gen_ptr = next_gen_ptr as *mut u8;
    let new_buf = unsafe {slice:: from_raw_parts_mut(next_gen_ptr,byte_size)};

    for i in 0..byte_size {
        let pos = i / 4;
        let x = pos % width;
        let y = pos / width;
        if x < width && y < height {
            let neighbour_count =  count_neighbours(buf, width, x , y);
            let x1 = x as i32;
            let y1 = y as i32;
            let w1 = width as i32;
            let is_alive = is_filled(buf, w1, x1, y1);
            if is_alive && neighbour_count < 2 {
                //TODO: Change signature to return the next state here. Do not modify the pointer.
                clear_cell(new_buf, width, x, y);
            } else if is_alive && neighbour_count == 2 || neighbour_count == 3 {
                set_pixel(new_buf, width, x, y );
            } else if is_alive && neighbour_count > 3 {
                clear_cell(new_buf, width, x, y);
            } else if !is_alive && neighbour_count == 3 {
                set_pixel(new_buf, width, x, y );
            } else {
                clear_cell(new_buf, width, x, y);
            }
        }
    }
    next_gen_ptr as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clear_sets_zero() {
        let h = 512 as usize;
        let w = 512 as usize;
        let byte_size = h * w * 4;
        let mut buf = Vec::with_capacity(byte_size);
        let ptr = buf.as_mut_ptr();
        let u_ptr = ptr as *mut u8;
        clear(u_ptr, w, h);
        let buf = unsafe { slice::from_raw_parts_mut(u_ptr, byte_size) };
        for i in buf.iter_mut() {
            assert_eq!(*i, 0);
        }
    }

//TODO: Find how Rust handles test description.
    #[test]
    fn next_generation_returns_the_same_if_unset() {
        let h = 512 as usize;
        let w = 512 as usize;
        let byte_size = h * w * 4;
        let mut buf = Vec::with_capacity(byte_size);
        let ptr = buf.as_mut_ptr();
        let u_ptr = ptr as *mut u8;
        let nxt_gen = next_generation(u_ptr, w, h);
        let nxt_gen = nxt_gen as *mut u8;
        let buf = unsafe { slice::from_raw_parts_mut(nxt_gen, byte_size) };
        for i in buf.iter_mut() {
            assert_eq!(*i, 0);
        }
    }
}