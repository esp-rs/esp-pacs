///Register `SLICE_HEADER_REMAIN` reader
pub type R = crate::R<SLICE_HEADER_REMAIN_SPEC>;
///Register `SLICE_HEADER_REMAIN` writer
pub type W = crate::W<SLICE_HEADER_REMAIN_SPEC>;
///Field `SLICE_REMAIN_BITLENGTH` reader - Configures Slice Header remain bit number
pub type SLICE_REMAIN_BITLENGTH_R = crate::FieldReader;
///Field `SLICE_REMAIN_BITLENGTH` writer - Configures Slice Header remain bit number
pub type SLICE_REMAIN_BITLENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SLICE_REMAIN_BIT` reader - Configures Slice Header remain bit
pub type SLICE_REMAIN_BIT_R = crate::FieldReader;
///Field `SLICE_REMAIN_BIT` writer - Configures Slice Header remain bit
pub type SLICE_REMAIN_BIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:2 - Configures Slice Header remain bit number
    #[inline(always)]
    pub fn slice_remain_bitlength(&self) -> SLICE_REMAIN_BITLENGTH_R {
        SLICE_REMAIN_BITLENGTH_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:10 - Configures Slice Header remain bit
    #[inline(always)]
    pub fn slice_remain_bit(&self) -> SLICE_REMAIN_BIT_R {
        SLICE_REMAIN_BIT_R::new(((self.bits >> 3) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLICE_HEADER_REMAIN")
            .field("slice_remain_bitlength", &self.slice_remain_bitlength())
            .field("slice_remain_bit", &self.slice_remain_bit())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Configures Slice Header remain bit number
    #[inline(always)]
    #[must_use]
    pub fn slice_remain_bitlength(&mut self) -> SLICE_REMAIN_BITLENGTH_W<SLICE_HEADER_REMAIN_SPEC> {
        SLICE_REMAIN_BITLENGTH_W::new(self, 0)
    }
    ///Bits 3:10 - Configures Slice Header remain bit
    #[inline(always)]
    #[must_use]
    pub fn slice_remain_bit(&mut self) -> SLICE_REMAIN_BIT_W<SLICE_HEADER_REMAIN_SPEC> {
        SLICE_REMAIN_BIT_W::new(self, 3)
    }
}
/**Frame Slice Header remain bit register.

You can [`read`](crate::generic::Reg::read) this register and get [`slice_header_remain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_remain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLICE_HEADER_REMAIN_SPEC;
impl crate::RegisterSpec for SLICE_HEADER_REMAIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slice_header_remain::R`](R) reader structure
impl crate::Readable for SLICE_HEADER_REMAIN_SPEC {}
///`write(|w| ..)` method takes [`slice_header_remain::W`](W) writer structure
impl crate::Writable for SLICE_HEADER_REMAIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLICE_HEADER_REMAIN to value 0
impl crate::Resettable for SLICE_HEADER_REMAIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
