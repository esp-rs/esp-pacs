#[doc = "Register `RC_STATUS1` reader"]
pub type R = crate::R<RC_STATUS1_SPEC>;
#[doc = "Field `FRAME_ENC_BITS` reader - Represents all MB actual encoding bits sum value of one frame."]
pub type FRAME_ENC_BITS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - Represents all MB actual encoding bits sum value of one frame."]
    #[inline(always)]
    pub fn frame_enc_bits(&self) -> FRAME_ENC_BITS_R {
        FRAME_ENC_BITS_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC_STATUS1")
            .field(
                "frame_enc_bits",
                &format_args!("{}", self.frame_enc_bits().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RC_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Rate control status register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC_STATUS1_SPEC;
impl crate::RegisterSpec for RC_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc_status1::R`](R) reader structure"]
impl crate::Readable for RC_STATUS1_SPEC {}
#[doc = "`reset()` method sets RC_STATUS1 to value 0"]
impl crate::Resettable for RC_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
