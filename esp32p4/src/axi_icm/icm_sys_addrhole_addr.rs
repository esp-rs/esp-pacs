#[doc = "Register `ICM_SYS_ADDRHOLE_ADDR` reader"]
pub type R = crate::R<ICM_SYS_ADDRHOLE_ADDR_SPEC>;
#[doc = "Field `ICM_SYS_ADDRHOLE_ADDR` reader - "]
pub type ICM_SYS_ADDRHOLE_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn icm_sys_addrhole_addr(&self) -> ICM_SYS_ADDRHOLE_ADDR_R {
        ICM_SYS_ADDRHOLE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_SYS_ADDRHOLE_ADDR")
            .field("icm_sys_addrhole_addr", &self.icm_sys_addrhole_addr())
            .finish()
    }
}
#[doc = "SYS address hole address\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_SYS_ADDRHOLE_ADDR_SPEC;
impl crate::RegisterSpec for ICM_SYS_ADDRHOLE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_sys_addrhole_addr::R`](R) reader structure"]
impl crate::Readable for ICM_SYS_ADDRHOLE_ADDR_SPEC {}
#[doc = "`reset()` method sets ICM_SYS_ADDRHOLE_ADDR to value 0"]
impl crate::Resettable for ICM_SYS_ADDRHOLE_ADDR_SPEC {}
