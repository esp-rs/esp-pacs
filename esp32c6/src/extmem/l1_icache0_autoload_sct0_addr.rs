#[doc = "Register `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` reader"]
pub type R = crate::R<L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` reader - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1_ICACHE0_AUTOLOAD_SCT0_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct0_addr(&self) -> L1_ICACHE0_AUTOLOAD_SCT0_ADDR_R {
        L1_ICACHE0_AUTOLOAD_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_AUTOLOAD_SCT0_ADDR")
            .field(
                "l1_icache0_autoload_sct0_addr",
                &format_args!("{}", self.l1_icache0_autoload_sct0_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L1 instruction Cache 0 autoload section 0 address configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_sct0_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_autoload_sct0_addr::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC {}
#[doc = "`reset()` method sets L1_ICACHE0_AUTOLOAD_SCT0_ADDR to value 0"]
impl crate::Resettable for L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
