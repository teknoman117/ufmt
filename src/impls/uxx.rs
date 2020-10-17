use core::str;

use crate::{uDebug, uDisplay, uDisplayHex, uWrite, Formatter};

macro_rules! uxx {
    ($n:expr, $buf:expr) => {{
        let mut n = $n;
        let mut i = $buf.len() - 1;
        loop {
            *$buf
                .get_mut(i)
                .unwrap_or_else(|| unsafe { assume_unreachable!() }) = (n % 10) as u8 + b'0';
            n = n / 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        unsafe { str::from_utf8_unchecked($buf.get(i..).unwrap_or_else(|| assume_unreachable!())) }
    }};
}

macro_rules! uxx_hex {
    ($n:expr, $buf:expr) => {{
        let mut n = $n;
        let mut i = $buf.len() - 1;

        loop {
            let d = (n % 16) as u8;
            *$buf
                .get_mut(i)
                .unwrap_or_else(|| unsafe { assume_unreachable!() }) = match d {
                0..=9 => d + b'0',
                10..=15 => (d - 10) + b'a',
                _ => b' ',
            };

            n = n / 16;

            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        unsafe { str::from_utf8_unchecked($buf.get(i..).unwrap_or_else(|| assume_unreachable!())) }
    }};
}

fn usize(n: usize, buf: &mut [u8]) -> &str {
    uxx!(n, buf)
}

fn usize_hex(n: usize, buf: &mut [u8]) -> &str {
    uxx_hex!(n, buf)
}

impl uDebug for u8 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 3] = unsafe { crate::uninitialized() };

        f.write_str(usize(usize::from(*self), &mut buf))
    }
}

impl uDisplay for u8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u8 as uDebug>::fmt(self, f)
    }
}

impl uDisplayHex for u8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 2] = unsafe { crate::uninitialized() };

        f.write_str(usize_hex(usize::from(*self), &mut buf))
    }
}

impl uDebug for u16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 5] = unsafe { crate::uninitialized() };

        f.write_str(usize(usize::from(*self), &mut buf))
    }
}

impl uDisplay for u16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u16 as uDebug>::fmt(self, f)
    }
}

impl uDisplayHex for u16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 4] = unsafe { crate::uninitialized() };

        f.write_str(usize_hex(usize::from(*self), &mut buf))
    }
}

impl uDebug for u32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 10] = unsafe { crate::uninitialized() };

        f.write_str(usize(*self as usize, &mut buf))
    }
}

impl uDisplay for u32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u32 as uDebug>::fmt(self, f)
    }
}

impl uDisplayHex for u32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 8] = unsafe { crate::uninitialized() };

        f.write_str(usize_hex(*self as usize, &mut buf))
    }
}

impl uDebug for u64 {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 20] = unsafe { crate::uninitialized() };

        let s = uxx!(*self, buf);
        f.write_str(s)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 20] = unsafe { crate::uninitialized() };

        f.write_str(usize(*self as usize, &mut buf))
    }
}

impl uDisplay for u64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u64 as uDebug>::fmt(self, f)
    }
}

impl uDisplayHex for u64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 16] = unsafe { crate::uninitialized() };

        f.write_str(usize_hex(*self as usize, &mut buf))
    }
}

impl uDebug for u128 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 39] = unsafe { crate::uninitialized() };

        let s = uxx!(*self, buf);
        f.write_str(s)
    }
}

impl uDisplay for u128 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u128 as uDebug>::fmt(self, f)
    }
}

impl uDisplayHex for u128 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf: [u8; 32] = unsafe { crate::uninitialized() };

        let s = uxx_hex!(*self, buf);
        f.write_str(s)
    }
}

impl uDebug for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u16 as uDebug>::fmt(&(*self as u16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u32 as uDebug>::fmt(&(*self as u32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u64 as uDebug>::fmt(&(*self as u64), f)
    }
}

impl uDisplay for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u16 as uDisplay>::fmt(&(*self as u16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u32 as uDisplay>::fmt(&(*self as u32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u64 as uDisplay>::fmt(&(*self as u64), f)
    }
}

impl uDisplayHex for usize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u16 as uDisplayHex>::fmt(&(*self as u16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u32 as uDisplayHex>::fmt(&(*self as u32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <u64 as uDisplayHex>::fmt(&(*self as u64), f)
    }
}
