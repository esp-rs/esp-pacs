#[doc = "Register `REDCY_SIG1` reader"]
pub struct R(crate::R<REDCY_SIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDCY_SIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDCY_SIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDCY_SIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDCY_SIG1` writer"]
pub struct W(crate::W<REDCY_SIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDCY_SIG1_SPEC>;
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
impl From<crate::W<REDCY_SIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDCY_SIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_SIG1` reader - reg_redcy_sig1"]
pub type REDCY_SIG1_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_SIG1` writer - reg_redcy_sig1"]
pub type REDCY_SIG1_W<'a, const O: u8> = crate::FieldWriter<'a, REDCY_SIG1_SPEC, 31, O, u32>;
#[doc = "Field `REDCY_NANDOR` reader - reg_redcy_nandor"]
pub type REDCY_NANDOR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - reg_redcy_sig1"]
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - reg_redcy_nandor"]
    #[inline(always)]
    pub fn redcy_nandor(&self) -> REDCY_NANDOR_R {
        REDCY_NANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDCY_SIG1")
            .field("redcy_sig1", &format_args!("{}", self.redcy_sig1().bits()))
            .field(
                "redcy_nandor",
                &format_args!("{}", self.redcy_nandor().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDCY_SIG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:30 - reg_redcy_sig1"]
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W<0> {
        REDCY_SIG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_REDCY_SIG1_REG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redcy_sig1](index.html) module"]
pub struct REDCY_SIG1_SPEC;
impl crate::RegisterSpec for REDCY_SIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redcy_sig1::R](R) reader structure"]
impl crate::Readable for REDCY_SIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redcy_sig1::W](W) writer structure"]
impl crate::Writable for REDCY_SIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REDCY_SIG1 to value 0"]
impl crate::Resettable for REDCY_SIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
