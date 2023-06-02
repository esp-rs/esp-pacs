#[doc = "Register `ENDIAN` reader"]
pub struct R(crate::R<ENDIAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDIAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDIAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDIAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDIAN` writer"]
pub struct W(crate::W<ENDIAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDIAN_SPEC>;
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
impl From<crate::W<ENDIAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDIAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDIAN` reader - Defines the endianness of input and output texts. &amp; \\[1:0\\] key endian # \\[3:2\\] text_in endian or in_stream endian # \\[5:4\\] text_out endian or out_stream endian # &amp;"]
pub type ENDIAN_R = crate::FieldReader;
#[doc = "Field `ENDIAN` writer - Defines the endianness of input and output texts. &amp; \\[1:0\\] key endian # \\[3:2\\] text_in endian or in_stream endian # \\[5:4\\] text_out endian or out_stream endian # &amp;"]
pub type ENDIAN_W<'a, const O: u8> = crate::FieldWriter<'a, ENDIAN_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Defines the endianness of input and output texts. &amp; \\[1:0\\] key endian # \\[3:2\\] text_in endian or in_stream endian # \\[5:4\\] text_out endian or out_stream endian # &amp;"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDIAN")
            .field("endian", &format_args!("{}", self.endian().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENDIAN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Defines the endianness of input and output texts. &amp; \\[1:0\\] key endian # \\[3:2\\] text_in endian or in_stream endian # \\[5:4\\] text_out endian or out_stream endian # &amp;"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<0> {
        ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endian configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endian](index.html) module"]
pub struct ENDIAN_SPEC;
impl crate::RegisterSpec for ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endian::R](R) reader structure"]
impl crate::Readable for ENDIAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endian::W](W) writer structure"]
impl crate::Writable for ENDIAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDIAN to value 0"]
impl crate::Resettable for ENDIAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
