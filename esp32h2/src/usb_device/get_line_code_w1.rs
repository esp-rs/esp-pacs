#[doc = "Register `GET_LINE_CODE_W1` reader"]
pub struct R(crate::R<GET_LINE_CODE_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GET_LINE_CODE_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GET_LINE_CODE_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GET_LINE_CODE_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GET_LINE_CODE_W1` writer"]
pub struct W(crate::W<GET_LINE_CODE_W1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GET_LINE_CODE_W1_SPEC>;
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
impl From<crate::W<GET_LINE_CODE_W1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GET_LINE_CODE_W1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GET_BDATA_BITS` reader - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type GET_BDATA_BITS_R = crate::FieldReader;
#[doc = "Field `GET_BDATA_BITS` writer - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type GET_BDATA_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, GET_LINE_CODE_W1_SPEC, 8, O>;
#[doc = "Field `GET_BPARITY_TYPE` reader - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type GET_BPARITY_TYPE_R = crate::FieldReader;
#[doc = "Field `GET_BPARITY_TYPE` writer - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type GET_BPARITY_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, GET_LINE_CODE_W1_SPEC, 8, O>;
#[doc = "Field `GET_BCHAR_FORMAT` reader - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type GET_BCHAR_FORMAT_R = crate::FieldReader;
#[doc = "Field `GET_BCHAR_FORMAT` writer - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type GET_BCHAR_FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, GET_LINE_CODE_W1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bdata_bits(&self) -> GET_BDATA_BITS_R {
        GET_BDATA_BITS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bparity_type(&self) -> GET_BPARITY_TYPE_R {
        GET_BPARITY_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bchar_format(&self) -> GET_BCHAR_FORMAT_R {
        GET_BCHAR_FORMAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GET_LINE_CODE_W1")
            .field(
                "get_bdata_bits",
                &format_args!("{}", self.get_bdata_bits().bits()),
            )
            .field(
                "get_bparity_type",
                &format_args!("{}", self.get_bparity_type().bits()),
            )
            .field(
                "get_bchar_format",
                &format_args!("{}", self.get_bchar_format().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GET_LINE_CODE_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    #[must_use]
    pub fn get_bdata_bits(&mut self) -> GET_BDATA_BITS_W<0> {
        GET_BDATA_BITS_W::new(self)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    #[must_use]
    pub fn get_bparity_type(&mut self) -> GET_BPARITY_TYPE_W<8> {
        GET_BPARITY_TYPE_W::new(self)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    #[must_use]
    pub fn get_bchar_format(&mut self) -> GET_BCHAR_FORMAT_W<16> {
        GET_BCHAR_FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "W1 of GET_LINE_CODING command.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [get_line_code_w1](index.html) module"]
pub struct GET_LINE_CODE_W1_SPEC;
impl crate::RegisterSpec for GET_LINE_CODE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [get_line_code_w1::R](R) reader structure"]
impl crate::Readable for GET_LINE_CODE_W1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [get_line_code_w1::W](W) writer structure"]
impl crate::Writable for GET_LINE_CODE_W1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GET_LINE_CODE_W1 to value 0"]
impl crate::Resettable for GET_LINE_CODE_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
