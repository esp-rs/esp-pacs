#[doc = "Register `BLK10_W10` reader"]
pub type R = crate::R<BLK10_W10_SPEC>;
#[doc = "Field `BLOCK19_W10` reader - Otp block10 word10 data."]
pub type BLOCK19_W10_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word10 data."]
    #[inline(always)]
    pub fn block19_w10(&self) -> BLOCK19_W10_R {
        BLOCK19_W10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK10_W10")
            .field(
                "block19_w10",
                &format_args!("{}", self.block19_w10().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK10_W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block10 data register10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk10_w10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK10_W10_SPEC;
impl crate::RegisterSpec for BLK10_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk10_w10::R`](R) reader structure"]
impl crate::Readable for BLK10_W10_SPEC {}
#[doc = "`reset()` method sets BLK10_W10 to value 0"]
impl crate::Resettable for BLK10_W10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
