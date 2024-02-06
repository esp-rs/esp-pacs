#[doc = "Register `BLK2_W7` reader"]
pub type R = crate::R<BLK2_W7_SPEC>;
#[doc = "Field `BLOCK2_W7` reader - Otp block2 word7 data."]
pub type BLOCK2_W7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word7 data."]
    #[inline(always)]
    pub fn block2_w7(&self) -> BLOCK2_W7_R {
        BLOCK2_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_W7")
            .field("block2_w7", &format_args!("{}", self.block2_w7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block2 data register7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_w7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_W7_SPEC;
impl crate::RegisterSpec for BLK2_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_w7::R`](R) reader structure"]
impl crate::Readable for BLK2_W7_SPEC {}
#[doc = "`reset()` method sets BLK2_W7 to value 0"]
impl crate::Resettable for BLK2_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
