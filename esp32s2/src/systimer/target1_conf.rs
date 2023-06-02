#[doc = "Register `TARGET1_CONF` reader"]
pub struct R(crate::R<TARGET1_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET1_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET1_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET1_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET1_CONF` writer"]
pub struct W(crate::W<TARGET1_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET1_CONF_SPEC>;
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
impl From<crate::W<TARGET1_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET1_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET1_PERIOD` reader - Set alarm period for system timer target 1, only valid in periodic alarms mode."]
pub type TARGET1_PERIOD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TARGET1_PERIOD` writer - Set alarm period for system timer target 1, only valid in periodic alarms mode."]
pub type TARGET1_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, TARGET1_CONF_SPEC, 30, O, u32, u32>;
#[doc = "Field `TARGET1_PERIOD_MODE` reader - Set work mode for system timer target 1. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET1_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET1_PERIOD_MODE` writer - Set work mode for system timer target 1. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET1_PERIOD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, TARGET1_CONF_SPEC, O>;
#[doc = "Field `TARGET1_WORK_EN` reader - System timer target 1 work enable."]
pub type TARGET1_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET1_WORK_EN` writer - System timer target 1 work enable."]
pub type TARGET1_WORK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TARGET1_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 1, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn target1_period(&self) -> TARGET1_PERIOD_R {
        TARGET1_PERIOD_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 1. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn target1_period_mode(&self) -> TARGET1_PERIOD_MODE_R {
        TARGET1_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System timer target 1 work enable."]
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET1_WORK_EN_R {
        TARGET1_WORK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET1_CONF")
            .field(
                "target1_period",
                &format_args!("{}", self.target1_period().bits()),
            )
            .field(
                "target1_period_mode",
                &format_args!("{}", self.target1_period_mode().bit()),
            )
            .field(
                "target1_work_en",
                &format_args!("{}", self.target1_work_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET1_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 1, only valid in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target1_period(&mut self) -> TARGET1_PERIOD_W<0> {
        TARGET1_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 1. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target1_period_mode(&mut self) -> TARGET1_PERIOD_MODE_W<30> {
        TARGET1_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bit 31 - System timer target 1 work enable."]
    #[inline(always)]
    #[must_use]
    pub fn target1_work_en(&mut self) -> TARGET1_WORK_EN_W<31> {
        TARGET1_WORK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure work mode for system timer target 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target1_conf](index.html) module"]
pub struct TARGET1_CONF_SPEC;
impl crate::RegisterSpec for TARGET1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target1_conf::R](R) reader structure"]
impl crate::Readable for TARGET1_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target1_conf::W](W) writer structure"]
impl crate::Writable for TARGET1_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET1_CONF to value 0"]
impl crate::Resettable for TARGET1_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
