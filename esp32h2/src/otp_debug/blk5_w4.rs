#[doc = "Register `BLK5_W4` reader"]
pub type R = crate::R<BLK5_W4_SPEC>;
#[doc = "Field `BLOCK5_W4` reader - Otp block5 word4 data."]
pub type BLOCK5_W4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word4 data."]
    #[inline(always)]
    pub fn block5_w4(&self) -> BLOCK5_W4_R {
        BLOCK5_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK5_W4")
            .field("block5_w4", &format_args!("{}", self.block5_w4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK5_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block5 data register4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk5_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK5_W4_SPEC;
impl crate::RegisterSpec for BLK5_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk5_w4::R`](R) reader structure"]
impl crate::Readable for BLK5_W4_SPEC {}
#[doc = "`reset()` method sets BLK5_W4 to value 0"]
impl crate::Resettable for BLK5_W4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
