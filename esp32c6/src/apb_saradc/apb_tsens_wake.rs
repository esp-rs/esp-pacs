#[doc = "Register `APB_TSENS_WAKE` reader"]
pub struct R(crate::R<APB_TSENS_WAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_WAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_WAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_WAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_WAKE` writer"]
pub struct W(crate::W<APB_TSENS_WAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_WAKE_SPEC>;
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
impl From<crate::W<APB_TSENS_WAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_WAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_TH_LOW` reader - reg_wakeup_th_low"]
pub type WAKEUP_TH_LOW_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_LOW` writer - reg_wakeup_th_low"]
pub type WAKEUP_TH_LOW_W<'a, const O: u8> = crate::FieldWriter<'a, APB_TSENS_WAKE_SPEC, 8, O>;
#[doc = "Field `WAKEUP_TH_HIGH` reader - reg_wakeup_th_high"]
pub type WAKEUP_TH_HIGH_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_HIGH` writer - reg_wakeup_th_high"]
pub type WAKEUP_TH_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, APB_TSENS_WAKE_SPEC, 8, O>;
#[doc = "Field `WAKEUP_OVER_UPPER_TH` reader - reg_wakeup_over_upper_th"]
pub type WAKEUP_OVER_UPPER_TH_R = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` reader - reg_wakeup_mode"]
pub type WAKEUP_MODE_R = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` writer - reg_wakeup_mode"]
pub type WAKEUP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_WAKE_SPEC, O>;
#[doc = "Field `WAKEUP_EN` reader - reg_wakeup_en"]
pub type WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WAKEUP_EN` writer - reg_wakeup_en"]
pub type WAKEUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_TSENS_WAKE_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - reg_wakeup_th_low"]
    #[inline(always)]
    pub fn wakeup_th_low(&self) -> WAKEUP_TH_LOW_R {
        WAKEUP_TH_LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reg_wakeup_th_high"]
    #[inline(always)]
    pub fn wakeup_th_high(&self) -> WAKEUP_TH_HIGH_R {
        WAKEUP_TH_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - reg_wakeup_over_upper_th"]
    #[inline(always)]
    pub fn wakeup_over_upper_th(&self) -> WAKEUP_OVER_UPPER_TH_R {
        WAKEUP_OVER_UPPER_TH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_wakeup_mode"]
    #[inline(always)]
    pub fn wakeup_mode(&self) -> WAKEUP_MODE_R {
        WAKEUP_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_wakeup_en"]
    #[inline(always)]
    pub fn wakeup_en(&self) -> WAKEUP_EN_R {
        WAKEUP_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_WAKE")
            .field(
                "wakeup_th_low",
                &format_args!("{}", self.wakeup_th_low().bits()),
            )
            .field(
                "wakeup_th_high",
                &format_args!("{}", self.wakeup_th_high().bits()),
            )
            .field(
                "wakeup_over_upper_th",
                &format_args!("{}", self.wakeup_over_upper_th().bit()),
            )
            .field("wakeup_mode", &format_args!("{}", self.wakeup_mode().bit()))
            .field("wakeup_en", &format_args!("{}", self.wakeup_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_WAKE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_wakeup_th_low"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_low(&mut self) -> WAKEUP_TH_LOW_W<0> {
        WAKEUP_TH_LOW_W::new(self)
    }
    #[doc = "Bits 8:15 - reg_wakeup_th_high"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_high(&mut self) -> WAKEUP_TH_HIGH_W<8> {
        WAKEUP_TH_HIGH_W::new(self)
    }
    #[doc = "Bit 17 - reg_wakeup_mode"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_mode(&mut self) -> WAKEUP_MODE_W<17> {
        WAKEUP_MODE_W::new(self)
    }
    #[doc = "Bit 18 - reg_wakeup_en"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_en(&mut self) -> WAKEUP_EN_W<18> {
        WAKEUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital tsens configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_wake](index.html) module"]
pub struct APB_TSENS_WAKE_SPEC;
impl crate::RegisterSpec for APB_TSENS_WAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_wake::R](R) reader structure"]
impl crate::Readable for APB_TSENS_WAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_wake::W](W) writer structure"]
impl crate::Writable for APB_TSENS_WAKE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_TSENS_WAKE to value 0xff00"]
impl crate::Resettable for APB_TSENS_WAKE_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
