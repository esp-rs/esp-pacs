#[doc = "Register `SET_LINE_CODE_W0` reader"]
pub struct R(crate::R<SET_LINE_CODE_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_LINE_CODE_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_LINE_CODE_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_LINE_CODE_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DW_DTE_RATE` reader - The value of dwDTERate set by host through SET_LINE_CODING command."]
pub type DW_DTE_RATE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value of dwDTERate set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn dw_dte_rate(&self) -> DW_DTE_RATE_R {
        DW_DTE_RATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_LINE_CODE_W0")
            .field(
                "dw_dte_rate",
                &format_args!("{}", self.dw_dte_rate().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_LINE_CODE_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "W0 of SET_LINE_CODING command.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_line_code_w0](index.html) module"]
pub struct SET_LINE_CODE_W0_SPEC;
impl crate::RegisterSpec for SET_LINE_CODE_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set_line_code_w0::R](R) reader structure"]
impl crate::Readable for SET_LINE_CODE_W0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SET_LINE_CODE_W0 to value 0"]
impl crate::Resettable for SET_LINE_CODE_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
