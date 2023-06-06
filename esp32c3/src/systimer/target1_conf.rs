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
#[doc = "Field `TARGET1_PERIOD` reader - target1 period"]
pub type TARGET1_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET1_PERIOD` writer - target1 period"]
pub type TARGET1_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, TARGET1_CONF_SPEC, 26, O, u32>;
#[doc = "Field `TARGET1_PERIOD_MODE` reader - Set target1 to period mode"]
pub type TARGET1_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET1_PERIOD_MODE` writer - Set target1 to period mode"]
pub type TARGET1_PERIOD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, TARGET1_CONF_SPEC, O>;
#[doc = "Field `TARGET1_TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TARGET1_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET1_TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TARGET1_TIMER_UNIT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TARGET1_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:25 - target1 period"]
    #[inline(always)]
    pub fn target1_period(&self) -> TARGET1_PERIOD_R {
        TARGET1_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target1 to period mode"]
    #[inline(always)]
    pub fn target1_period_mode(&self) -> TARGET1_PERIOD_MODE_R {
        TARGET1_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn target1_timer_unit_sel(&self) -> TARGET1_TIMER_UNIT_SEL_R {
        TARGET1_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
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
                "target1_timer_unit_sel",
                &format_args!("{}", self.target1_timer_unit_sel().bit()),
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
    #[doc = "Bits 0:25 - target1 period"]
    #[inline(always)]
    #[must_use]
    pub fn target1_period(&mut self) -> TARGET1_PERIOD_W<0> {
        TARGET1_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - Set target1 to period mode"]
    #[inline(always)]
    #[must_use]
    pub fn target1_period_mode(&mut self) -> TARGET1_PERIOD_MODE_W<30> {
        TARGET1_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    #[must_use]
    pub fn target1_timer_unit_sel(&mut self) -> TARGET1_TIMER_UNIT_SEL_W<31> {
        TARGET1_TIMER_UNIT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_TARGET1_CONF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target1_conf](index.html) module"]
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
