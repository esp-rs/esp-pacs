#[doc = "Register `PERI1_TIMEOUT_ADDR` reader"]
pub type R = crate::R<PERI1_TIMEOUT_ADDR_SPEC>;
#[doc = "Field `PERI1_TIMEOUT_ADDR` reader - Represents the address information of abnormal access."]
pub type PERI1_TIMEOUT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address information of abnormal access."]
    #[inline(always)]
    pub fn peri1_timeout_addr(&self) -> PERI1_TIMEOUT_ADDR_R {
        PERI1_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI1_TIMEOUT_ADDR")
            .field("peri1_timeout_addr", &self.peri1_timeout_addr())
            .finish()
    }
}
#[doc = "HP_PERI1_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri1_timeout_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI1_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for PERI1_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri1_timeout_addr::R`](R) reader structure"]
impl crate::Readable for PERI1_TIMEOUT_ADDR_SPEC {}
#[doc = "`reset()` method sets PERI1_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for PERI1_TIMEOUT_ADDR_SPEC {}
