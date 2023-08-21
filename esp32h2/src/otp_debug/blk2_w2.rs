#[doc = "Register `BLK2_W2` reader"]
pub type R = crate::R<BLK2_W2_SPEC>;
#[doc = "Field `BLOCK2_W2` reader - Otp block2 word2 data."]
pub type BLOCK2_W2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word2 data."]
    #[inline(always)]
    pub fn block2_w2(&self) -> BLOCK2_W2_R {
        BLOCK2_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_W2")
            .field("block2_w2", &format_args!("{}", self.block2_w2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block2 data register2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_w2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_W2_SPEC;
impl crate::RegisterSpec for BLK2_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_w2::R`](R) reader structure"]
impl crate::Readable for BLK2_W2_SPEC {}
#[doc = "`reset()` method sets BLK2_W2 to value 0"]
impl crate::Resettable for BLK2_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
