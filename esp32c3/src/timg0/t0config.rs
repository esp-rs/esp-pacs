#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USE_XTAL` reader - reg_t0_use_xtal."]
pub type USE_XTAL_R = crate::BitReader;
#[doc = "Field `USE_XTAL` writer - reg_t0_use_xtal."]
pub type USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
#[doc = "Field `ALARM_EN` reader - reg_t0_alarm_en."]
pub type ALARM_EN_R = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - reg_t0_alarm_en."]
pub type ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
#[doc = "Field `DIVCNT_RST` writer - reg_t0_divcnt_rst."]
pub type DIVCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
#[doc = "Field `DIVIDER` reader - reg_t0_divider."]
pub type DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - reg_t0_divider."]
pub type DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, T0CONFIG_SPEC, 16, O, u16>;
#[doc = "Field `AUTORELOAD` reader - reg_t0_autoreload."]
pub type AUTORELOAD_R = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - reg_t0_autoreload."]
pub type AUTORELOAD_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
#[doc = "Field `INCREASE` reader - reg_t0_increase."]
pub type INCREASE_R = crate::BitReader;
#[doc = "Field `INCREASE` writer - reg_t0_increase."]
pub type INCREASE_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
#[doc = "Field `EN` reader - reg_t0_en."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - reg_t0_en."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, T0CONFIG_SPEC, O>;
impl R {
    #[doc = "Bit 9 - reg_t0_use_xtal."]
    #[inline(always)]
    pub fn use_xtal(&self) -> USE_XTAL_R {
        USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_t0_alarm_en."]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - reg_t0_divider."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - reg_t0_autoreload."]
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_t0_increase."]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_t0_en."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CONFIG")
            .field("use_xtal", &format_args!("{}", self.use_xtal().bit()))
            .field("alarm_en", &format_args!("{}", self.alarm_en().bit()))
            .field("divider", &format_args!("{}", self.divider().bits()))
            .field("autoreload", &format_args!("{}", self.autoreload().bit()))
            .field("increase", &format_args!("{}", self.increase().bit()))
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 9 - reg_t0_use_xtal."]
    #[inline(always)]
    #[must_use]
    pub fn use_xtal(&mut self) -> USE_XTAL_W<9> {
        USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - reg_t0_alarm_en."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<10> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bit 12 - reg_t0_divcnt_rst."]
    #[inline(always)]
    #[must_use]
    pub fn divcnt_rst(&mut self) -> DIVCNT_RST_W<12> {
        DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 13:28 - reg_t0_divider."]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<13> {
        DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - reg_t0_autoreload."]
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<29> {
        AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - reg_t0_increase."]
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<30> {
        INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - reg_t0_en."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0CONFIG_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_2000;
}
