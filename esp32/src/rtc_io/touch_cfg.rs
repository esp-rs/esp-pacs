#[doc = "Register `TOUCH_CFG` reader"]
pub struct R(crate::R<TOUCH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CFG` writer"]
pub struct W(crate::W<TOUCH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CFG_SPEC>;
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
impl From<crate::W<TOUCH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_DCUR` reader - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub type TOUCH_DCUR_R = crate::FieldReader;
#[doc = "Field `TOUCH_DCUR` writer - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub type TOUCH_DCUR_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CFG_SPEC, 2, O>;
#[doc = "Field `TOUCH_DRANGE` reader - touch sensor saw wave voltage range."]
pub type TOUCH_DRANGE_R = crate::FieldReader;
#[doc = "Field `TOUCH_DRANGE` writer - touch sensor saw wave voltage range."]
pub type TOUCH_DRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CFG_SPEC, 2, O>;
#[doc = "Field `TOUCH_DREFL` reader - touch sensor saw wave bottom voltage."]
pub type TOUCH_DREFL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFL` writer - touch sensor saw wave bottom voltage."]
pub type TOUCH_DREFL_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CFG_SPEC, 2, O>;
#[doc = "Field `TOUCH_DREFH` reader - touch sensor saw wave top voltage."]
pub type TOUCH_DREFH_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFH` writer - touch sensor saw wave top voltage."]
pub type TOUCH_DREFH_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CFG_SPEC, 2, O>;
#[doc = "Field `TOUCH_XPD_BIAS` reader - touch sensor bias power on."]
pub type TOUCH_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `TOUCH_XPD_BIAS` writer - touch sensor bias power on."]
pub type TOUCH_XPD_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn touch_dcur(&self) -> TOUCH_DCUR_R {
        TOUCH_DCUR_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn touch_drange(&self) -> TOUCH_DRANGE_R {
        TOUCH_DRANGE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn touch_xpd_bias(&self) -> TOUCH_XPD_BIAS_R {
        TOUCH_XPD_BIAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CFG")
            .field("touch_dcur", &format_args!("{}", self.touch_dcur().bits()))
            .field(
                "touch_drange",
                &format_args!("{}", self.touch_drange().bits()),
            )
            .field(
                "touch_drefl",
                &format_args!("{}", self.touch_drefl().bits()),
            )
            .field(
                "touch_drefh",
                &format_args!("{}", self.touch_drefh().bits()),
            )
            .field(
                "touch_xpd_bias",
                &format_args!("{}", self.touch_xpd_bias().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    #[must_use]
    pub fn touch_dcur(&mut self) -> TOUCH_DCUR_W<23> {
        TOUCH_DCUR_W::new(self)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drange(&mut self) -> TOUCH_DRANGE_W<25> {
        TOUCH_DRANGE_W::new(self)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W<27> {
        TOUCH_DREFL_W::new(self)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    #[must_use]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W<29> {
        TOUCH_DREFH_W::new(self)
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd_bias(&mut self) -> TOUCH_XPD_BIAS_W<31> {
        TOUCH_XPD_BIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_cfg](index.html) module"]
pub struct TOUCH_CFG_SPEC;
impl crate::RegisterSpec for TOUCH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_cfg::R](R) reader structure"]
impl crate::Readable for TOUCH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_cfg::W](W) writer structure"]
impl crate::Writable for TOUCH_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_CFG to value 0x6600_0000"]
impl crate::Resettable for TOUCH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6600_0000;
}
