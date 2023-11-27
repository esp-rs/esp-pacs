#[doc = "Register `DPI_VCID_ACT` reader"]
pub type R = crate::R<DPI_VCID_ACT_SPEC>;
#[doc = "Field `DPI_VCID_ACT` reader - NA"]
pub type DPI_VCID_ACT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn dpi_vcid_act(&self) -> DPI_VCID_ACT_R {
        DPI_VCID_ACT_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_VCID_ACT")
            .field(
                "dpi_vcid_act",
                &format_args!("{}", self.dpi_vcid_act().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPI_VCID_ACT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_vcid_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_VCID_ACT_SPEC;
impl crate::RegisterSpec for DPI_VCID_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_vcid_act::R`](R) reader structure"]
impl crate::Readable for DPI_VCID_ACT_SPEC {}
#[doc = "`reset()` method sets DPI_VCID_ACT to value 0"]
impl crate::Resettable for DPI_VCID_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
