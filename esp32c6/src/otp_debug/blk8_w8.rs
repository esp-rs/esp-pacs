#[doc = "Register `BLK8_W8` reader"]
pub type R = crate::R<BLK8_W8_SPEC>;
#[doc = "Field `BLOCK8_W8` reader - Otp block8 word8 data."]
pub type BLOCK8_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word8 data."]
    #[inline(always)]
    pub fn block8_w8(&self) -> BLOCK8_W8_R {
        BLOCK8_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK8_W8")
            .field("block8_w8", &format_args!("{}", self.block8_w8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK8_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block8 data register8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk8_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK8_W8_SPEC;
impl crate::RegisterSpec for BLK8_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk8_w8::R`](R) reader structure"]
impl crate::Readable for BLK8_W8_SPEC {}
#[doc = "`reset()` method sets BLK8_W8 to value 0"]
impl crate::Resettable for BLK8_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
