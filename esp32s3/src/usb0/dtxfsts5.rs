#[doc = "Register `DTXFSTS5` reader"]
pub struct R(crate::R<DTXFSTS5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INEPTXFSPCAVAIL5` reader - "]
pub type D_INEPTXFSPCAVAIL5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail5(&self) -> D_INEPTXFSPCAVAIL5_R {
        D_INEPTXFSPCAVAIL5_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS5")
            .field(
                "d_ineptxfspcavail5",
                &format_args!("{}", self.d_ineptxfspcavail5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts5](index.html) module"]
pub struct DTXFSTS5_SPEC;
impl crate::RegisterSpec for DTXFSTS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts5::R](R) reader structure"]
impl crate::Readable for DTXFSTS5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS5 to value 0"]
impl crate::Resettable for DTXFSTS5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
