#[doc = "Register `BLK3_RDATA4` reader"]
pub type R = crate::R<BLK3_RDATA4_SPEC>;
#[doc = "Field `RD_SECURE_VERSION` reader - "]
pub type RD_SECURE_VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_secure_version(&self) -> RD_SECURE_VERSION_R {
        RD_SECURE_VERSION_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA4")
            .field("rd_secure_version", &self.rd_secure_version())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA4_SPEC;
impl crate::RegisterSpec for BLK3_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata4::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA4_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA4 to value 0"]
impl crate::Resettable for BLK3_RDATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
