#[doc = "Register `PMS_REJECT_ADDR` reader"]
pub type R = crate::R<PMS_REJECT_ADDR_SPEC>;
#[doc = "Field `REJECT_ADDR` reader - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type REJECT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn reject_addr(&self) -> REJECT_ADDR_R {
        REJECT_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMS_REJECT_ADDR")
            .field("reject_addr", &self.reject_addr())
            .finish()
    }
}
#[doc = "SPI1 access reject addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMS_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for PMS_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pms_reject_addr::R`](R) reader structure"]
impl crate::Readable for PMS_REJECT_ADDR_SPEC {}
#[doc = "`reset()` method sets PMS_REJECT_ADDR to value 0"]
impl crate::Resettable for PMS_REJECT_ADDR_SPEC {}
