#[doc = "Register `TRACEMEM_MUX_MODE` reader"]
pub struct R(crate::R<TRACEMEM_MUX_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEMEM_MUX_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEMEM_MUX_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEMEM_MUX_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEMEM_MUX_MODE` writer"]
pub struct W(crate::W<TRACEMEM_MUX_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEMEM_MUX_MODE_SPEC>;
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
impl From<crate::W<TRACEMEM_MUX_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEMEM_MUX_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEMEM_MUX_MODE` reader - "]
pub type TRACEMEM_MUX_MODE_R = crate::FieldReader;
#[doc = "Field `TRACEMEM_MUX_MODE` writer - "]
pub type TRACEMEM_MUX_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, TRACEMEM_MUX_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tracemem_mux_mode(&self) -> TRACEMEM_MUX_MODE_R {
        TRACEMEM_MUX_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACEMEM_MUX_MODE")
            .field(
                "tracemem_mux_mode",
                &format_args!("{}", self.tracemem_mux_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRACEMEM_MUX_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tracemem_mux_mode(&mut self) -> TRACEMEM_MUX_MODE_W<0> {
        TRACEMEM_MUX_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tracemem_mux_mode](index.html) module"]
pub struct TRACEMEM_MUX_MODE_SPEC;
impl crate::RegisterSpec for TRACEMEM_MUX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tracemem_mux_mode::R](R) reader structure"]
impl crate::Readable for TRACEMEM_MUX_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tracemem_mux_mode::W](W) writer structure"]
impl crate::Writable for TRACEMEM_MUX_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACEMEM_MUX_MODE to value 0"]
impl crate::Resettable for TRACEMEM_MUX_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
