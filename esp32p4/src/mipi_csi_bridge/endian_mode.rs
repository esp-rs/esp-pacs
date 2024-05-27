///Register `ENDIAN_MODE` reader
pub type R = crate::R<ENDIAN_MODE_SPEC>;
///Register `ENDIAN_MODE` writer
pub type W = crate::W<ENDIAN_MODE_SPEC>;
///Field `BYTE_ENDIAN_ORDER` reader - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
pub type BYTE_ENDIAN_ORDER_R = crate::BitReader;
///Field `BYTE_ENDIAN_ORDER` writer - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
pub type BYTE_ENDIAN_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIT_ENDIAN_ORDER` reader - N/A
pub type BIT_ENDIAN_ORDER_R = crate::BitReader;
///Field `BIT_ENDIAN_ORDER` writer - N/A
pub type BIT_ENDIAN_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    #[inline(always)]
    pub fn byte_endian_order(&self) -> BYTE_ENDIAN_ORDER_R {
        BYTE_ENDIAN_ORDER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - N/A
    #[inline(always)]
    pub fn bit_endian_order(&self) -> BIT_ENDIAN_ORDER_R {
        BIT_ENDIAN_ORDER_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDIAN_MODE")
            .field("byte_endian_order", &self.byte_endian_order())
            .field("bit_endian_order", &self.bit_endian_order())
            .finish()
    }
}
impl W {
    ///Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    #[inline(always)]
    #[must_use]
    pub fn byte_endian_order(&mut self) -> BYTE_ENDIAN_ORDER_W<ENDIAN_MODE_SPEC> {
        BYTE_ENDIAN_ORDER_W::new(self, 0)
    }
    ///Bit 1 - N/A
    #[inline(always)]
    #[must_use]
    pub fn bit_endian_order(&mut self) -> BIT_ENDIAN_ORDER_W<ENDIAN_MODE_SPEC> {
        BIT_ENDIAN_ORDER_W::new(self, 1)
    }
}
/**data endianness order configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`endian_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENDIAN_MODE_SPEC;
impl crate::RegisterSpec for ENDIAN_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`endian_mode::R`](R) reader structure
impl crate::Readable for ENDIAN_MODE_SPEC {}
///`write(|w| ..)` method takes [`endian_mode::W`](W) writer structure
impl crate::Writable for ENDIAN_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENDIAN_MODE to value 0
impl crate::Resettable for ENDIAN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
