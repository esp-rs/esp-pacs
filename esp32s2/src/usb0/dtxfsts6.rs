#[doc = "Register `DTXFSTS6` reader"]
pub type R = crate::R<DTXFSTS6_SPEC>;
#[doc = "Field `D_INEPTXFSPCAVAIL6` reader - "]
pub type D_INEPTXFSPCAVAIL6_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail6(&self) -> D_INEPTXFSPCAVAIL6_R {
        D_INEPTXFSPCAVAIL6_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS6")
            .field(
                "d_ineptxfspcavail6",
                &format_args!("{}", self.d_ineptxfspcavail6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS6_SPEC;
impl crate::RegisterSpec for DTXFSTS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts6::R`](R) reader structure"]
impl crate::Readable for DTXFSTS6_SPEC {}
#[doc = "`reset()` method sets DTXFSTS6 to value 0"]
impl crate::Resettable for DTXFSTS6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
