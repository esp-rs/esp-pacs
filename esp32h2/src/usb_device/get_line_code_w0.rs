#[doc = "Register `GET_LINE_CODE_W0` reader"]
pub struct R(crate::R<GET_LINE_CODE_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GET_LINE_CODE_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GET_LINE_CODE_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GET_LINE_CODE_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GET_LINE_CODE_W0` writer"]
pub struct W(crate::W<GET_LINE_CODE_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GET_LINE_CODE_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GET_LINE_CODE_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GET_LINE_CODE_W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GET_DW_DTE_RATE` reader - The value of dwDTERate set by software which is requested by GET_LINE_CODING command."]
pub type GET_DW_DTE_RATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GET_DW_DTE_RATE` writer - The value of dwDTERate set by software which is requested by GET_LINE_CODING command."]
pub type GET_DW_DTE_RATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, GET_LINE_CODE_W0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value of dwDTERate set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_dw_dte_rate(&self) -> GET_DW_DTE_RATE_R {
        GET_DW_DTE_RATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GET_LINE_CODE_W0")
            .field(
                "get_dw_dte_rate",
                &format_args!("{}", self.get_dw_dte_rate().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GET_LINE_CODE_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value of dwDTERate set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    #[must_use]
    pub fn get_dw_dte_rate(&mut self) -> GET_DW_DTE_RATE_W<0> {
        GET_DW_DTE_RATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "W0 of GET_LINE_CODING command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [get_line_code_w0](index.html) module"]
pub struct GET_LINE_CODE_W0_SPEC;
impl crate::RegisterSpec for GET_LINE_CODE_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [get_line_code_w0::R](R) reader structure"]
impl crate::Readable for GET_LINE_CODE_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [get_line_code_w0::W](W) writer structure"]
impl crate::Writable for GET_LINE_CODE_W0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GET_LINE_CODE_W0 to value 0"]
impl crate::Resettable for GET_LINE_CODE_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
