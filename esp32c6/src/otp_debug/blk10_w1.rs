#[doc = "Register `BLK10_W1` reader"]
pub type R = crate::R<BLK10_W1_SPEC>;
#[doc = "Field `BLOCK10_W1` reader - Otp block10 word1 data."]
pub type BLOCK10_W1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word1 data."]
    #[inline(always)]
    pub fn block10_w1(&self) -> BLOCK10_W1_R {
        BLOCK10_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK10_W1")
            .field("block10_w1", &self.block10_w1())
            .finish()
    }
}
#[doc = "Otp debuger block10 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk10_w1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK10_W1_SPEC;
impl crate::RegisterSpec for BLK10_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk10_w1::R`](R) reader structure"]
impl crate::Readable for BLK10_W1_SPEC {}
#[doc = "`reset()` method sets BLK10_W1 to value 0"]
impl crate::Resettable for BLK10_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
