#[doc = "Register `REDCY_SIG0` reader"]
pub struct R(crate::R<REDCY_SIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDCY_SIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDCY_SIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDCY_SIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDCY_SIG0` writer"]
pub struct W(crate::W<REDCY_SIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDCY_SIG0_SPEC>;
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
impl From<crate::W<REDCY_SIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDCY_SIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_SIG0` reader - reg_redcy_sig0"]
pub type REDCY_SIG0_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_SIG0` writer - reg_redcy_sig0"]
pub type REDCY_SIG0_W<'a, const O: u8> = crate::FieldWriter<'a, REDCY_SIG0_SPEC, 31, O, u32>;
#[doc = "Field `REDCY_ANDOR` reader - reg_redcy_andor"]
pub type REDCY_ANDOR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    pub fn redcy_sig0(&self) -> REDCY_SIG0_R {
        REDCY_SIG0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - reg_redcy_andor"]
    #[inline(always)]
    pub fn redcy_andor(&self) -> REDCY_ANDOR_R {
        REDCY_ANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDCY_SIG0")
            .field("redcy_sig0", &format_args!("{}", self.redcy_sig0().bits()))
            .field("redcy_andor", &format_args!("{}", self.redcy_andor().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDCY_SIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:30 - reg_redcy_sig0"]
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig0(&mut self) -> REDCY_SIG0_W<0> {
        REDCY_SIG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_REDCY_SIG0_REG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig0](index.html) module"]
pub struct REDCY_SIG0_SPEC;
impl crate::RegisterSpec for REDCY_SIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redcy_sig0::R](R) reader structure"]
impl crate::Readable for REDCY_SIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redcy_sig0::W](W) writer structure"]
impl crate::Writable for REDCY_SIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REDCY_SIG0 to value 0"]
impl crate::Resettable for REDCY_SIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
