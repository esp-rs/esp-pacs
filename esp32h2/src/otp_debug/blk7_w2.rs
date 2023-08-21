#[doc = "Register `BLK7_W2` reader"]
pub type R = crate::R<BLK7_W2_SPEC>;
#[doc = "Field `BLOCK7_W2` reader - Otp block7 word2 data."]
pub type BLOCK7_W2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block7 word2 data."]
    #[inline(always)]
    pub fn block7_w2(&self) -> BLOCK7_W2_R {
        BLOCK7_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W2")
            .field("block7_w2", &format_args!("{}", self.block7_w2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK7_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block7 data register2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk7_w2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK7_W2_SPEC;
impl crate::RegisterSpec for BLK7_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk7_w2::R`](R) reader structure"]
impl crate::Readable for BLK7_W2_SPEC {}
#[doc = "`reset()` method sets BLK7_W2 to value 0"]
impl crate::Resettable for BLK7_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
