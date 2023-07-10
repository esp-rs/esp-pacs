#[doc = "Register `BOD_MODE1_CNTL` reader"]
pub struct R(crate::R<BOD_MODE1_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_MODE1_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_MODE1_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_MODE1_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD_MODE1_CNTL` writer"]
pub struct W(crate::W<BOD_MODE1_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_MODE1_CNTL_SPEC>;
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
impl From<crate::W<BOD_MODE1_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_MODE1_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_MODE1_RESET_ENA` reader - need_des"]
pub type BOD_MODE1_RESET_ENA_R = crate::BitReader;
#[doc = "Field `BOD_MODE1_RESET_ENA` writer - need_des"]
pub type BOD_MODE1_RESET_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BOD_MODE1_CNTL_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode1_reset_ena(&self) -> BOD_MODE1_RESET_ENA_R {
        BOD_MODE1_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD_MODE1_CNTL")
            .field(
                "bod_mode1_reset_ena",
                &format_args!("{}", self.bod_mode1_reset_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BOD_MODE1_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode1_reset_ena(&mut self) -> BOD_MODE1_RESET_ENA_W<31> {
        BOD_MODE1_RESET_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod_mode1_cntl](index.html) module"]
pub struct BOD_MODE1_CNTL_SPEC;
impl crate::RegisterSpec for BOD_MODE1_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod_mode1_cntl::R](R) reader structure"]
impl crate::Readable for BOD_MODE1_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod_mode1_cntl::W](W) writer structure"]
impl crate::Writable for BOD_MODE1_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD_MODE1_CNTL to value 0"]
impl crate::Resettable for BOD_MODE1_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
