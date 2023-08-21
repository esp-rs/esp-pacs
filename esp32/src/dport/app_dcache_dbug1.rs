#[doc = "Register `APP_DCACHE_DBUG1` reader"]
pub type R = crate::R<APP_DCACHE_DBUG1_SPEC>;
#[doc = "Field `APP_CTAG_RAM_RDATA` reader - "]
pub type APP_CTAG_RAM_RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_ctag_ram_rdata(&self) -> APP_CTAG_RAM_RDATA_R {
        APP_CTAG_RAM_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG1")
            .field(
                "app_ctag_ram_rdata",
                &format_args!("{}", self.app_ctag_ram_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_DCACHE_DBUG1_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_dcache_dbug1::R`](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG1_SPEC {}
#[doc = "`reset()` method sets APP_DCACHE_DBUG1 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
