#[doc = "Register `L1_ICACHE2_AUTOLOAD_SCT0_SIZE` reader"]
pub struct R(crate::R<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1_ICACHE2_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct0_size(&self) -> L1_ICACHE2_AUTOLOAD_SCT0_SIZE_R {
        L1_ICACHE2_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE2_AUTOLOAD_SCT0_SIZE")
            .field(
                "l1_icache2_autoload_sct0_size",
                &format_args!("{}", self.l1_icache2_autoload_sct0_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1 instruction Cache 2 autoload section 0 size configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache2_autoload_sct0_size](index.html) module"]
pub struct L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache2_autoload_sct0_size::R](R) reader structure"]
impl crate::Readable for L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
