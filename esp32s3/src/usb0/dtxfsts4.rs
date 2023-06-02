#[doc = "Register `DTXFSTS4` reader"]
pub struct R(crate::R<DTXFSTS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INEPTXFSPCAVAIL4` reader - "]
pub type D_INEPTXFSPCAVAIL4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn d_ineptxfspcavail4(&self) -> D_INEPTXFSPCAVAIL4_R {
        D_INEPTXFSPCAVAIL4_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS4")
            .field(
                "d_ineptxfspcavail4",
                &format_args!("{}", self.d_ineptxfspcavail4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts4](index.html) module"]
pub struct DTXFSTS4_SPEC;
impl crate::RegisterSpec for DTXFSTS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts4::R](R) reader structure"]
impl crate::Readable for DTXFSTS4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS4 to value 0"]
impl crate::Resettable for DTXFSTS4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
