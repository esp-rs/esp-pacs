#[doc = "Register `LP_PERI_TIMEOUT_ADDR` reader"]
pub type R = crate::R<LP_PERI_TIMEOUT_ADDR_SPEC>;
#[doc = "Field `LP_PERI_TIMEOUT_ADDR` reader - "]
pub type LP_PERI_TIMEOUT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lp_peri_timeout_addr(&self) -> LP_PERI_TIMEOUT_ADDR_R {
        LP_PERI_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI_TIMEOUT_ADDR")
            .field("lp_peri_timeout_addr", &self.lp_peri_timeout_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for LP_PERI_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_peri_timeout_addr::R`](R) reader structure"]
impl crate::Readable for LP_PERI_TIMEOUT_ADDR_SPEC {}
#[doc = "`reset()` method sets LP_PERI_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for LP_PERI_TIMEOUT_ADDR_SPEC {}
