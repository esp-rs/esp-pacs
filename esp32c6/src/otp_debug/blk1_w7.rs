#[doc = "Register `BLK1_W7` reader"]
pub type R = crate::R<BLK1_W7_SPEC>;
#[doc = "Field `BLOCK1_W7` reader - Otp block1 word7 data."]
pub type BLOCK1_W7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block1 word7 data."]
    #[inline(always)]
    pub fn block1_w7(&self) -> BLOCK1_W7_R {
        BLOCK1_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W7")
            .field("block1_w7", &format_args!("{}", self.block1_w7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK1_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block1 data register7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_w7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_W7_SPEC;
impl crate::RegisterSpec for BLK1_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_w7::R`](R) reader structure"]
impl crate::Readable for BLK1_W7_SPEC {}
#[doc = "`reset()` method sets BLK1_W7 to value 0"]
impl crate::Resettable for BLK1_W7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
