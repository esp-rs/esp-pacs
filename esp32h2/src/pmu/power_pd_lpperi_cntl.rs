#[doc = "Register `POWER_PD_LPPERI_CNTL` reader"]
pub struct R(crate::R<POWER_PD_LPPERI_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_PD_LPPERI_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_PD_LPPERI_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_PD_LPPERI_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_PD_LPPERI_CNTL` writer"]
pub struct W(crate::W<POWER_PD_LPPERI_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_PD_LPPERI_CNTL_SPEC>;
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
impl From<crate::W<POWER_PD_LPPERI_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_PD_LPPERI_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_LP_PERI_RESET` reader - need_des"]
pub type FORCE_LP_PERI_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_RESET` writer - need_des"]
pub type FORCE_LP_PERI_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_LP_PERI_ISO` reader - need_des"]
pub type FORCE_LP_PERI_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_ISO` writer - need_des"]
pub type FORCE_LP_PERI_ISO_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_LP_PERI_PU` reader - need_des"]
pub type FORCE_LP_PERI_PU_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_PU` writer - need_des"]
pub type FORCE_LP_PERI_PU_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_LP_PERI_NO_RESET` reader - need_des"]
pub type FORCE_LP_PERI_NO_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_NO_RESET` writer - need_des"]
pub type FORCE_LP_PERI_NO_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_LP_PERI_NO_ISO` reader - need_des"]
pub type FORCE_LP_PERI_NO_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_NO_ISO` writer - need_des"]
pub type FORCE_LP_PERI_NO_ISO_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_LP_PERI_PD` reader - need_des"]
pub type FORCE_LP_PERI_PD_R = crate::BitReader;
#[doc = "Field `FORCE_LP_PERI_PD` writer - need_des"]
pub type FORCE_LP_PERI_PD_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_LPPERI_CNTL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_reset(&self) -> FORCE_LP_PERI_RESET_R {
        FORCE_LP_PERI_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_iso(&self) -> FORCE_LP_PERI_ISO_R {
        FORCE_LP_PERI_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pu(&self) -> FORCE_LP_PERI_PU_R {
        FORCE_LP_PERI_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_reset(&self) -> FORCE_LP_PERI_NO_RESET_R {
        FORCE_LP_PERI_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_no_iso(&self) -> FORCE_LP_PERI_NO_ISO_R {
        FORCE_LP_PERI_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_lp_peri_pd(&self) -> FORCE_LP_PERI_PD_R {
        FORCE_LP_PERI_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_LPPERI_CNTL")
            .field(
                "force_lp_peri_reset",
                &format_args!("{}", self.force_lp_peri_reset().bit()),
            )
            .field(
                "force_lp_peri_iso",
                &format_args!("{}", self.force_lp_peri_iso().bit()),
            )
            .field(
                "force_lp_peri_pu",
                &format_args!("{}", self.force_lp_peri_pu().bit()),
            )
            .field(
                "force_lp_peri_no_reset",
                &format_args!("{}", self.force_lp_peri_no_reset().bit()),
            )
            .field(
                "force_lp_peri_no_iso",
                &format_args!("{}", self.force_lp_peri_no_iso().bit()),
            )
            .field(
                "force_lp_peri_pd",
                &format_args!("{}", self.force_lp_peri_pd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_LPPERI_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_reset(&mut self) -> FORCE_LP_PERI_RESET_W<0> {
        FORCE_LP_PERI_RESET_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_iso(&mut self) -> FORCE_LP_PERI_ISO_W<1> {
        FORCE_LP_PERI_ISO_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_pu(&mut self) -> FORCE_LP_PERI_PU_W<2> {
        FORCE_LP_PERI_PU_W::new(self)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_no_reset(&mut self) -> FORCE_LP_PERI_NO_RESET_W<3> {
        FORCE_LP_PERI_NO_RESET_W::new(self)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_no_iso(&mut self) -> FORCE_LP_PERI_NO_ISO_W<4> {
        FORCE_LP_PERI_NO_ISO_W::new(self)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_lp_peri_pd(&mut self) -> FORCE_LP_PERI_PD_W<5> {
        FORCE_LP_PERI_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_pd_lpperi_cntl](index.html) module"]
pub struct POWER_PD_LPPERI_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_LPPERI_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_pd_lpperi_cntl::R](R) reader structure"]
impl crate::Readable for POWER_PD_LPPERI_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_pd_lpperi_cntl::W](W) writer structure"]
impl crate::Writable for POWER_PD_LPPERI_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_PD_LPPERI_CNTL to value 0x1c"]
impl crate::Resettable for POWER_PD_LPPERI_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c;
}
