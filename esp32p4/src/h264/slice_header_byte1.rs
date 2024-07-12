#[doc = "Register `SLICE_HEADER_BYTE1` reader"]
pub type R = crate::R<SLICE_HEADER_BYTE1_SPEC>;
#[doc = "Register `SLICE_HEADER_BYTE1` writer"]
pub type W = crate::W<SLICE_HEADER_BYTE1_SPEC>;
#[doc = "Field `SLICE_BYTE_MSB` reader - Configures Slice Header high 32 bit"]
pub type SLICE_BYTE_MSB_R = crate::FieldReader<u32>;
#[doc = "Field `SLICE_BYTE_MSB` writer - Configures Slice Header high 32 bit"]
pub type SLICE_BYTE_MSB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures Slice Header high 32 bit"]
    #[inline(always)]
    pub fn slice_byte_msb(&self) -> SLICE_BYTE_MSB_R {
        SLICE_BYTE_MSB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLICE_HEADER_BYTE1")
            .field("slice_byte_msb", &self.slice_byte_msb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures Slice Header high 32 bit"]
    #[inline(always)]
    #[must_use]
    pub fn slice_byte_msb(&mut self) -> SLICE_BYTE_MSB_W<SLICE_HEADER_BYTE1_SPEC> {
        SLICE_BYTE_MSB_W::new(self, 0)
    }
}
#[doc = "Frame Slice Header byte high 32 bit register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slice_header_byte1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slice_header_byte1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLICE_HEADER_BYTE1_SPEC;
impl crate::RegisterSpec for SLICE_HEADER_BYTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slice_header_byte1::R`](R) reader structure"]
impl crate::Readable for SLICE_HEADER_BYTE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slice_header_byte1::W`](W) writer structure"]
impl crate::Writable for SLICE_HEADER_BYTE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLICE_HEADER_BYTE1 to value 0"]
impl crate::Resettable for SLICE_HEADER_BYTE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
