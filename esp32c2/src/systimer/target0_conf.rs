#[doc = "Register `TARGET0_CONF` reader"]
pub struct R(crate::R<TARGET0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET0_CONF` writer"]
pub struct W(crate::W<TARGET0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET0_CONF_SPEC>;
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
impl From<crate::W<TARGET0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET0_PERIOD` reader - target0 period"]
pub type TARGET0_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET0_PERIOD` writer - target0 period"]
pub type TARGET0_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, TARGET0_CONF_SPEC, 26, O, u32>;
#[doc = "Field `TARGET0_PERIOD_MODE` reader - Set target0 to period mode"]
pub type TARGET0_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET0_PERIOD_MODE` writer - Set target0 to period mode"]
pub type TARGET0_PERIOD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, TARGET0_CONF_SPEC, O>;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TARGET0_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TARGET0_TIMER_UNIT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TARGET0_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    pub fn target0_period(&self) -> TARGET0_PERIOD_R {
        TARGET0_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    pub fn target0_period_mode(&self) -> TARGET0_PERIOD_MODE_R {
        TARGET0_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn target0_timer_unit_sel(&self) -> TARGET0_TIMER_UNIT_SEL_R {
        TARGET0_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_CONF")
            .field(
                "target0_period",
                &format_args!("{}", self.target0_period().bits()),
            )
            .field(
                "target0_period_mode",
                &format_args!("{}", self.target0_period_mode().bit()),
            )
            .field(
                "target0_timer_unit_sel",
                &format_args!("{}", self.target0_timer_unit_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    #[must_use]
    pub fn target0_period(&mut self) -> TARGET0_PERIOD_W<0> {
        TARGET0_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    #[must_use]
    pub fn target0_period_mode(&mut self) -> TARGET0_PERIOD_MODE_W<30> {
        TARGET0_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    #[must_use]
    pub fn target0_timer_unit_sel(&mut self) -> TARGET0_TIMER_UNIT_SEL_W<31> {
        TARGET0_TIMER_UNIT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer comp0 target mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target0_conf](index.html) module"]
pub struct TARGET0_CONF_SPEC;
impl crate::RegisterSpec for TARGET0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target0_conf::R](R) reader structure"]
impl crate::Readable for TARGET0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target0_conf::W](W) writer structure"]
impl crate::Writable for TARGET0_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET0_CONF to value 0"]
impl crate::Resettable for TARGET0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
