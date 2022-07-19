#[doc = "Register `RTC_CPU_PERIOD_CONF` reader"]
pub struct R(crate::R<RTC_CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CPU_PERIOD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CPU_PERIOD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CPU_PERIOD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CPU_PERIOD_CONF` writer"]
pub struct W(crate::W<RTC_CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CPU_PERIOD_CONF_SPEC>;
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
impl From<crate::W<RTC_CPU_PERIOD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CPU_PERIOD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CPUSEL_CONF` reader - CPU sel option"]
pub type RTC_CPUSEL_CONF_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CPUSEL_CONF` writer - CPU sel option"]
pub type RTC_CPUSEL_CONF_W<'a> = crate::BitWriter<'a, u32, RTC_CPU_PERIOD_CONF_SPEC, bool, 29>;
#[doc = "Field `RTC_CPUPERIOD_SEL` reader - Need add desc"]
pub type RTC_CPUPERIOD_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_CPUPERIOD_SEL` writer - Need add desc"]
pub type RTC_CPUPERIOD_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CPU_PERIOD_CONF_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn rtc_cpusel_conf(&self) -> RTC_CPUSEL_CONF_R {
        RTC_CPUSEL_CONF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_cpuperiod_sel(&self) -> RTC_CPUPERIOD_SEL_R {
        RTC_CPUPERIOD_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn rtc_cpusel_conf(&mut self) -> RTC_CPUSEL_CONF_W {
        RTC_CPUSEL_CONF_W::new(self)
    }
    #[doc = "Bits 30:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_cpuperiod_sel(&mut self) -> RTC_CPUPERIOD_SEL_W {
        RTC_CPUPERIOD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cpu_period_conf](index.html) module"]
pub struct RTC_CPU_PERIOD_CONF_SPEC;
impl crate::RegisterSpec for RTC_CPU_PERIOD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cpu_period_conf::R](R) reader structure"]
impl crate::Readable for RTC_CPU_PERIOD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cpu_period_conf::W](W) writer structure"]
impl crate::Writable for RTC_CPU_PERIOD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CPU_PERIOD_CONF to value 0"]
impl crate::Resettable for RTC_CPU_PERIOD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
