#[doc = "Register `BLK10_W6` reader"]
pub type R = crate::R<BLK10_W6_SPEC>;
#[doc = "Field `BLOCK10_W6` reader - Otp block10 word6 data."]
pub type BLOCK10_W6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word6 data."]
    #[inline(always)]
    pub fn block10_w6(&self) -> BLOCK10_W6_R {
        BLOCK10_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK10_W6")
            .field("block10_w6", &format_args!("{}", self.block10_w6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK10_W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block10 data register6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk10_w6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK10_W6_SPEC;
impl crate::RegisterSpec for BLK10_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk10_w6::R`](R) reader structure"]
impl crate::Readable for BLK10_W6_SPEC {}
#[doc = "`reset()` method sets BLK10_W6 to value 0"]
impl crate::Resettable for BLK10_W6_SPEC {
    const RESET_VALUE: u32 = 0;
}
