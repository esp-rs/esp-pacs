#[doc = "Register `PERI0_TIMEOUT_ADDR` reader"]
pub type R = crate::R<PERI0_TIMEOUT_ADDR_SPEC>;
#[doc = "Field `PERI0_TIMEOUT_ADDR` reader - Represents the address information of abnormal access."]
pub type PERI0_TIMEOUT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address information of abnormal access."]
    #[inline(always)]
    pub fn peri0_timeout_addr(&self) -> PERI0_TIMEOUT_ADDR_R {
        PERI0_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI0_TIMEOUT_ADDR")
            .field("peri0_timeout_addr", &self.peri0_timeout_addr())
            .finish()
    }
}
#[doc = "HP_PERI0_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri0_timeout_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI0_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for PERI0_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri0_timeout_addr::R`](R) reader structure"]
impl crate::Readable for PERI0_TIMEOUT_ADDR_SPEC {}
#[doc = "`reset()` method sets PERI0_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for PERI0_TIMEOUT_ADDR_SPEC {}
