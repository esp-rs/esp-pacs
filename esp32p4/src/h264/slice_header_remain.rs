#[doc = "Register `SLICE_HEADER_REMAIN` reader"]
pub type R = crate::R<SLICE_HEADER_REMAIN_SPEC>;
#[doc = "Register `SLICE_HEADER_REMAIN` writer"]
pub type W = crate::W<SLICE_HEADER_REMAIN_SPEC>;
#[doc = "Field `SLICE_REMAIN_BITLENGTH` reader - Configures Slice Header remain bit number"]
pub type SLICE_REMAIN_BITLENGTH_R = crate::FieldReader;
#[doc = "Field `SLICE_REMAIN_BITLENGTH` writer - Configures Slice Header remain bit number"]
pub type SLICE_REMAIN_BITLENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLICE_REMAIN_BIT` reader - Configures Slice Header remain bit"]
pub type SLICE_REMAIN_BIT_R = crate::FieldReader;
#[doc = "Field `SLICE_REMAIN_BIT` writer - Configures Slice Header remain bit"]
pub type SLICE_REMAIN_BIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Configures Slice Header remain bit number"]
    #[inline(always)]
    pub fn slice_remain_bitlength(&self) -> SLICE_REMAIN_BITLENGTH_R {
        SLICE_REMAIN_BITLENGTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:10 - Configures Slice Header remain bit"]
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
    #[doc = "Bits 0:2 - Configures Slice Header remain bit number"]
    #[inline(always)]
    pub fn slice_remain_bitlength(&mut self) -> SLICE_REMAIN_BITLENGTH_W<SLICE_HEADER_REMAIN_SPEC> {
        SLICE_REMAIN_BITLENGTH_W::new(self, 0)
    }
    #[doc = "Bits 3:10 - Configures Slice Header remain bit"]
    #[inline(always)]
    pub fn slice_remain_bit(&mut self) -> SLICE_REMAIN_BIT_W<SLICE_HEADER_REMAIN_SPEC> {
        SLICE_REMAIN_BIT_W::new(self, 3)
    }
}
#[doc = "Frame Slice Header remain bit register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slice_header_remain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slice_header_remain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLICE_HEADER_REMAIN_SPEC;
impl crate::RegisterSpec for SLICE_HEADER_REMAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slice_header_remain::R`](R) reader structure"]
impl crate::Readable for SLICE_HEADER_REMAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slice_header_remain::W`](W) writer structure"]
impl crate::Writable for SLICE_HEADER_REMAIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLICE_HEADER_REMAIN to value 0"]
impl crate::Resettable for SLICE_HEADER_REMAIN_SPEC {}
