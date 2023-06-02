#[doc = "Register `L1_ICACHE2_AUTOLOAD_SCT1_ADDR` reader"]
pub struct R(crate::R<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_ICACHE2_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct1_addr(&self) -> L1_ICACHE2_AUTOLOAD_SCT1_ADDR_R {
        L1_ICACHE2_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE2_AUTOLOAD_SCT1_ADDR")
            .field(
                "l1_icache2_autoload_sct1_addr",
                &format_args!("{}", self.l1_icache2_autoload_sct1_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1 instruction Cache 2 autoload section 1 address configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache2_autoload_sct1_addr](index.html) module"]
pub struct L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache2_autoload_sct1_addr::R](R) reader structure"]
impl crate::Readable for L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
