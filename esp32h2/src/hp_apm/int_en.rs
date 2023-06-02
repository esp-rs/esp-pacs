#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0_APM_INT_EN` reader - APM M0 interrupt enable"]
pub type M0_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M0_APM_INT_EN` writer - APM M0 interrupt enable"]
pub type M0_APM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, INT_EN_SPEC, O>;
#[doc = "Field `M1_APM_INT_EN` reader - APM M1 interrupt enable"]
pub type M1_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M1_APM_INT_EN` writer - APM M1 interrupt enable"]
pub type M1_APM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, INT_EN_SPEC, O>;
#[doc = "Field `M2_APM_INT_EN` reader - APM M2 interrupt enable"]
pub type M2_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M2_APM_INT_EN` writer - APM M2 interrupt enable"]
pub type M2_APM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, INT_EN_SPEC, O>;
#[doc = "Field `M3_APM_INT_EN` reader - APM M3 interrupt enable"]
pub type M3_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M3_APM_INT_EN` writer - APM M3 interrupt enable"]
pub type M3_APM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, INT_EN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - APM M0 interrupt enable"]
    #[inline(always)]
    pub fn m0_apm_int_en(&self) -> M0_APM_INT_EN_R {
        M0_APM_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APM M1 interrupt enable"]
    #[inline(always)]
    pub fn m1_apm_int_en(&self) -> M1_APM_INT_EN_R {
        M1_APM_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APM M2 interrupt enable"]
    #[inline(always)]
    pub fn m2_apm_int_en(&self) -> M2_APM_INT_EN_R {
        M2_APM_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APM M3 interrupt enable"]
    #[inline(always)]
    pub fn m3_apm_int_en(&self) -> M3_APM_INT_EN_R {
        M3_APM_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_EN")
            .field(
                "m0_apm_int_en",
                &format_args!("{}", self.m0_apm_int_en().bit()),
            )
            .field(
                "m1_apm_int_en",
                &format_args!("{}", self.m1_apm_int_en().bit()),
            )
            .field(
                "m2_apm_int_en",
                &format_args!("{}", self.m2_apm_int_en().bit()),
            )
            .field(
                "m3_apm_int_en",
                &format_args!("{}", self.m3_apm_int_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - APM M0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0_apm_int_en(&mut self) -> M0_APM_INT_EN_W<0> {
        M0_APM_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - APM M1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1_apm_int_en(&mut self) -> M1_APM_INT_EN_W<1> {
        M1_APM_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - APM M2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m2_apm_int_en(&mut self) -> M2_APM_INT_EN_W<2> {
        M2_APM_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - APM M3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m3_apm_int_en(&mut self) -> M3_APM_INT_EN_W<3> {
        M3_APM_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
