#[doc = "Register `BLK6_W5` reader"]
pub type R = crate::R<BLK6_W5_SPEC>;
#[doc = "Field `BLOCK6_W5` reader - Otp block6 word5 data."]
pub type BLOCK6_W5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block6 word5 data."]
    #[inline(always)]
    pub fn block6_w5(&self) -> BLOCK6_W5_R {
        BLOCK6_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK6_W5")
            .field("block6_w5", &format_args!("{}", self.block6_w5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK6_W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block6 data register5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk6_w5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK6_W5_SPEC;
impl crate::RegisterSpec for BLK6_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk6_w5::R`](R) reader structure"]
impl crate::Readable for BLK6_W5_SPEC {}
#[doc = "`reset()` method sets BLK6_W5 to value 0"]
impl crate::Resettable for BLK6_W5_SPEC {
    const RESET_VALUE: u32 = 0;
}
