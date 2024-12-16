#[doc = "Register `SLICE_HEADER_BYTE_LENGTH` reader"]
pub type R = crate::R<SLICE_HEADER_BYTE_LENGTH_SPEC>;
#[doc = "Register `SLICE_HEADER_BYTE_LENGTH` writer"]
pub type W = crate::W<SLICE_HEADER_BYTE_LENGTH_SPEC>;
#[doc = "Field `SLICE_BYTE_LENGTH` reader - Configures Slice Header byte number"]
pub type SLICE_BYTE_LENGTH_R = crate::FieldReader;
#[doc = "Field `SLICE_BYTE_LENGTH` writer - Configures Slice Header byte number"]
pub type SLICE_BYTE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures Slice Header byte number"]
    #[inline(always)]
    pub fn slice_byte_length(&self) -> SLICE_BYTE_LENGTH_R {
        SLICE_BYTE_LENGTH_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLICE_HEADER_BYTE_LENGTH")
            .field("slice_byte_length", &self.slice_byte_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures Slice Header byte number"]
    #[inline(always)]
    pub fn slice_byte_length(&mut self) -> SLICE_BYTE_LENGTH_W<SLICE_HEADER_BYTE_LENGTH_SPEC> {
        SLICE_BYTE_LENGTH_W::new(self, 0)
    }
}
#[doc = "Frame Slice Header byte length register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slice_header_byte_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slice_header_byte_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLICE_HEADER_BYTE_LENGTH_SPEC;
impl crate::RegisterSpec for SLICE_HEADER_BYTE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slice_header_byte_length::R`](R) reader structure"]
impl crate::Readable for SLICE_HEADER_BYTE_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slice_header_byte_length::W`](W) writer structure"]
impl crate::Writable for SLICE_HEADER_BYTE_LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLICE_HEADER_BYTE_LENGTH to value 0"]
impl crate::Resettable for SLICE_HEADER_BYTE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
