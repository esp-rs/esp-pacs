#[doc = "Register `BLK3_W1` reader"]
pub type R = crate::R<BLK3_W1_SPEC>;
#[doc = "Field `BLOCK3_W1` reader - Otp block3 word1 data."]
pub type BLOCK3_W1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word1 data."]
    #[inline(always)]
    pub fn block3_w1(&self) -> BLOCK3_W1_R {
        BLOCK3_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_W1")
            .field("block3_w1", &format_args!("{}", self.block3_w1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block3 data register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_W1_SPEC;
impl crate::RegisterSpec for BLK3_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_w1::R`](R) reader structure"]
impl crate::Readable for BLK3_W1_SPEC {}
#[doc = "`reset()` method sets BLK3_W1 to value 0"]
impl crate::Resettable for BLK3_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
