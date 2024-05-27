#[doc = "Register `BLK5_W11` reader"]
pub type R = crate::R<BLK5_W11_SPEC>;
#[doc = "Field `BLOCK5_W11` reader - Otp block5 word11 data."]
pub type BLOCK5_W11_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word11 data."]
    #[inline(always)]
    pub fn block5_w11(&self) -> BLOCK5_W11_R {
        BLOCK5_W11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK5_W11")
            .field("block5_w11", &self.block5_w11())
            .finish()
    }
}
#[doc = "Otp debuger block5 data register11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk5_w11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK5_W11_SPEC;
impl crate::RegisterSpec for BLK5_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk5_w11::R`](R) reader structure"]
impl crate::Readable for BLK5_W11_SPEC {}
#[doc = "`reset()` method sets BLK5_W11 to value 0"]
impl crate::Resettable for BLK5_W11_SPEC {
    const RESET_VALUE: u32 = 0;
}
