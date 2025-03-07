#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::mem;

pub use crate::error::{Error, ErrorType};
use crate::stringparse::*;
use crate::Deserializer;
pub use crate::Result;

impl<'de> Deserializer<'de> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn parse_str_(&mut self) -> Result<&'de str> {
        // Add 1 to skip the initial "
        let idx = self.iidx + 1;
        let mut padding = [0_u8; 32];
        //let mut read: usize = 0;

        // we include the terminal '"' so we know where to end
        // This is safe since we check sub's lenght in the range access above and only
        // create sub sliced form sub to `sub.len()`.

        let src: &[u8] = unsafe { &self.input.get_unchecked(idx..) };
        let mut src_i: usize = 0;
        let mut len = src_i;
        loop {
            let v: __m128i = if src.len() >= src_i + 16 {
                // This is safe since we ensure src is at least 16 wide
                #[allow(clippy::cast_ptr_alignment)]
                unsafe {
                    _mm_loadu_si128(src.as_ptr().add(src_i) as *const __m128i)
                }
            } else {
                unsafe {
                    padding
                        .get_unchecked_mut(..src.len() - src_i)
                        .clone_from_slice(src.get_unchecked(src_i..));
                    // This is safe since we ensure src is at least 32 wide
                    #[allow(clippy::cast_ptr_alignment)]
                    _mm_loadu_si128(padding.as_ptr() as *const __m128i)
                }
            };

            // store to dest unconditionally - we can overwrite the bits we don't like
            // later
            let bs_bits: u32 = unsafe {
                static_cast_u32!(_mm_movemask_epi8(_mm_cmpeq_epi8(
                    v,
                    #[allow(clippy::cast_possible_wrap)]
                    _mm_set1_epi8(b'\\' as i8)
                )))
            };
            #[allow(clippy::cast_possible_wrap)]
            let quote_mask = unsafe { _mm_cmpeq_epi8(v, _mm_set1_epi8(b'"' as i8)) };
            let quote_bits = unsafe { static_cast_u32!(_mm_movemask_epi8(quote_mask)) };
            if (bs_bits.wrapping_sub(1) & quote_bits) != 0 {
                // we encountered quotes first. Move dst to point to quotes and exit
                // find out where the quote is...
                let quote_dist: u32 = quote_bits.trailing_zeros();

                ///////////////////////
                // Above, check for overflow in case someone has a crazy string (>=4GB?)
                // But only add the overflow check when the document itself exceeds 4GB
                // Currently unneeded because we refuse to parse docs larger or equal to 4GB.
                ////////////////////////

                // we advance the point, accounting for the fact that we have a NULl termination

                len += quote_dist as usize;
                unsafe {
                    let v = self.input.get_unchecked(idx..idx + len) as *const [u8] as *const str;
                    return Ok(&*v);
                }

                // we compare the pointers since we care if they are 'at the same spot'
                // not if they are the same value
            }
            if (quote_bits.wrapping_sub(1) & bs_bits) == 0 {
                // they are the same. Since they can't co-occur, it means we encountered
                // neither.
                src_i += 16;
                len += 16;
            } else {
                // Move to the 'bad' character
                let bs_dist: u32 = bs_bits.trailing_zeros();
                len += bs_dist as usize;
                src_i += bs_dist as usize;
                break;
            }
        }

        let mut dst_i: usize = 0;
        let dst: &mut [u8] = &mut self.strings;

        // To be more conform with upstream
        #[allow(clippy::if_not_else)]
        loop {
            let v: __m128i = if src.len() >= src_i + 16 {
                // This is safe since we ensure src is at least 16 wide
                #[allow(clippy::cast_ptr_alignment)]
                unsafe {
                    _mm_loadu_si128(src.as_ptr().add(src_i) as *const __m128i)
                }
            } else {
                unsafe {
                    padding
                        .get_unchecked_mut(..src.len() - src_i)
                        .clone_from_slice(src.get_unchecked(src_i..));
                    // This is safe since we ensure src is at least 16 wide
                    #[allow(clippy::cast_ptr_alignment)]
                    _mm_loadu_si128(padding.as_ptr() as *const __m128i)
                }
            };

            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                _mm_storeu_si128(dst.as_mut_ptr().add(dst_i) as *mut __m128i, v)
            };

            // store to dest unconditionally - we can overwrite the bits we don't like
            // later
            let bs_bits: u32 = unsafe {
                static_cast_u32!(_mm_movemask_epi8(_mm_cmpeq_epi8(
                    v,
                    #[allow(clippy::cast_possible_wrap)]
                    _mm_set1_epi8(b'\\' as i8)
                )))
            };
            #[allow(clippy::cast_possible_wrap)]
            let quote_mask = unsafe { _mm_cmpeq_epi8(v, _mm_set1_epi8(b'"' as i8)) };
            let quote_bits = unsafe { static_cast_u32!(_mm_movemask_epi8(quote_mask)) };
            if (bs_bits.wrapping_sub(1) & quote_bits) != 0 {
                // we encountered quotes first. Move dst to point to quotes and exit
                // find out where the quote is...
                let quote_dist: u32 = quote_bits.trailing_zeros();

                ///////////////////////
                // Above, check for overflow in case someone has a crazy string (>=4GB?)
                // But only add the overflow check when the document itself exceeds 4GB
                // Currently unneeded because we refuse to parse docs larger or equal to 4GB.
                ////////////////////////

                // we advance the point, accounting for the fact that we have a NULl termination

                dst_i += quote_dist as usize;
                unsafe {
                    self.input
                        .get_unchecked_mut(idx + len..idx + len + dst_i)
                        .clone_from_slice(&self.strings.get_unchecked(..dst_i));
                    let v = self.input.get_unchecked(idx..idx + len + dst_i) as *const [u8]
                        as *const str;
                    self.str_offset += dst_i as usize;
                    return Ok(&*v);
                }

                // we compare the pointers since we care if they are 'at the same spot'
                // not if they are the same value
            }
            if (quote_bits.wrapping_sub(1) & bs_bits) != 0 {
                // find out where the backspace is
                let bs_dist: u32 = bs_bits.trailing_zeros();
                let escape_char: u8 = unsafe { *src.get_unchecked(src_i + bs_dist as usize + 1) };
                // we encountered backslash first. Handle backslash
                if escape_char == b'u' {
                    // move src/dst up to the start; they will be further adjusted
                    // within the unicode codepoint handling code.
                    src_i += bs_dist as usize;
                    dst_i += bs_dist as usize;
                    let (o, s) = if let Ok(r) =
                        handle_unicode_codepoint(unsafe { src.get_unchecked(src_i..) }, unsafe {
                            dst.get_unchecked_mut(dst_i..)
                        }) {
                        r
                    } else {
                        return Err(self.error(ErrorType::InvlaidUnicodeCodepoint));
                    };
                    if o == 0 {
                        return Err(self.error(ErrorType::InvlaidUnicodeCodepoint));
                    };
                    // We moved o steps forword at the destiation and 6 on the source
                    src_i += s;
                    dst_i += o;
                } else {
                    // simple 1:1 conversion. Will eat bs_dist+2 characters in input and
                    // write bs_dist+1 characters to output
                    // note this may reach beyond the part of the buffer we've actually
                    // seen. I think this is ok
                    let escape_result: u8 =
                        unsafe { *ESCAPE_MAP.get_unchecked(escape_char as usize) };
                    if escape_result == 0 {
                        return Err(self.error(ErrorType::InvalidEscape));
                    }
                    unsafe {
                        *dst.get_unchecked_mut(dst_i + bs_dist as usize) = escape_result;
                    }
                    src_i += bs_dist as usize + 2;
                    dst_i += bs_dist as usize + 1;
                }
            } else {
                // they are the same. Since they can't co-occur, it means we encountered
                // neither.
                src_i += 16;
                dst_i += 16;
            }
        }
    }
}
