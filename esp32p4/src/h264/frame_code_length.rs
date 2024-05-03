#[doc = "Register `FRAME_CODE_LENGTH` reader"]
pub type R = crate::R<FRAME_CODE_LENGTH_SPEC>;
#[doc = "Field `FRAME_CODE_LENGTH` reader - Represents current frame code byte length."]
pub type FRAME_CODE_LENGTH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Represents current frame code byte length."]
    #[inline(always)]
    pub fn frame_code_length(&self) -> FRAME_CODE_LENGTH_R {
        FRAME_CODE_LENGTH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_CODE_LENGTH")
            .field("frame_code_length", &self.frame_code_length().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRAME_CODE_LENGTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Frame code byte length register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_code_length::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAME_CODE_LENGTH_SPEC;
impl crate::RegisterSpec for FRAME_CODE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_code_length::R`](R) reader structure"]
impl crate::Readable for FRAME_CODE_LENGTH_SPEC {}
#[doc = "`reset()` method sets FRAME_CODE_LENGTH to value 0"]
impl crate::Resettable for FRAME_CODE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
