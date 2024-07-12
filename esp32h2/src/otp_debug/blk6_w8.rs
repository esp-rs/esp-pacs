#[doc = "Register `BLK6_W8` reader"]
pub type R = crate::R<BLK6_W8_SPEC>;
#[doc = "Field `BLOCK6_W8` reader - Otp block6 word8 data."]
pub type BLOCK6_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block6 word8 data."]
    #[inline(always)]
    pub fn block6_w8(&self) -> BLOCK6_W8_R {
        BLOCK6_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK6_W8")
            .field("block6_w8", &self.block6_w8())
            .finish()
    }
}
#[doc = "Otp debuger block6 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk6_w8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK6_W8_SPEC;
impl crate::RegisterSpec for BLK6_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk6_w8::R`](R) reader structure"]
impl crate::Readable for BLK6_W8_SPEC {}
#[doc = "`reset()` method sets BLK6_W8 to value 0"]
impl crate::Resettable for BLK6_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
