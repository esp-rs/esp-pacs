#[doc = "Register `TARGET2_CONF` reader"]
pub struct R(crate::R<TARGET2_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET2_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET2_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET2_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET2_CONF` writer"]
pub struct W(crate::W<TARGET2_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET2_CONF_SPEC>;
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
impl From<crate::W<TARGET2_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET2_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET2_PERIOD` reader - target2 period"]
pub type TARGET2_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET2_PERIOD` writer - target2 period"]
pub type TARGET2_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, TARGET2_CONF_SPEC, 26, O, u32>;
#[doc = "Field `TARGET2_PERIOD_MODE` reader - Set target2 to period mode"]
pub type TARGET2_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET2_PERIOD_MODE` writer - Set target2 to period mode"]
pub type TARGET2_PERIOD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, TARGET2_CONF_SPEC, O>;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TARGET2_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TARGET2_TIMER_UNIT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TARGET2_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:25 - target2 period"]
    #[inline(always)]
    pub fn target2_period(&self) -> TARGET2_PERIOD_R {
        TARGET2_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target2 to period mode"]
    #[inline(always)]
    pub fn target2_period_mode(&self) -> TARGET2_PERIOD_MODE_R {
        TARGET2_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn target2_timer_unit_sel(&self) -> TARGET2_TIMER_UNIT_SEL_R {
        TARGET2_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_CONF")
            .field(
                "target2_period",
                &format_args!("{}", self.target2_period().bits()),
            )
            .field(
                "target2_period_mode",
                &format_args!("{}", self.target2_period_mode().bit()),
            )
            .field(
                "target2_timer_unit_sel",
                &format_args!("{}", self.target2_timer_unit_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET2_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:25 - target2 period"]
    #[inline(always)]
    #[must_use]
    pub fn target2_period(&mut self) -> TARGET2_PERIOD_W<0> {
        TARGET2_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - Set target2 to period mode"]
    #[inline(always)]
    #[must_use]
    pub fn target2_period_mode(&mut self) -> TARGET2_PERIOD_MODE_W<30> {
        TARGET2_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    #[must_use]
    pub fn target2_timer_unit_sel(&mut self) -> TARGET2_TIMER_UNIT_SEL_W<31> {
        TARGET2_TIMER_UNIT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_TARGET2_CONF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target2_conf](index.html) module"]
pub struct TARGET2_CONF_SPEC;
impl crate::RegisterSpec for TARGET2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target2_conf::R](R) reader structure"]
impl crate::Readable for TARGET2_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target2_conf::W](W) writer structure"]
impl crate::Writable for TARGET2_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET2_CONF to value 0"]
impl crate::Resettable for TARGET2_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
