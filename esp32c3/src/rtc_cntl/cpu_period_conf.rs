#[doc = "Register `CPU_PERIOD_CONF` reader"]
pub struct R(crate::R<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERIOD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERIOD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERIOD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERIOD_CONF` writer"]
pub struct W(crate::W<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERIOD_CONF_SPEC>;
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
impl From<crate::W<CPU_PERIOD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERIOD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUSEL_CONF` reader - CPU sel option"]
pub type CPUSEL_CONF_R = crate::BitReader;
#[doc = "Field `CPUSEL_CONF` writer - CPU sel option"]
pub type CPUSEL_CONF_W<'a, const O: u8> = crate::BitWriter<'a, CPU_PERIOD_CONF_SPEC, O>;
#[doc = "Field `CPUPERIOD_SEL` reader - CPU clk sel option"]
pub type CPUPERIOD_SEL_R = crate::FieldReader;
#[doc = "Field `CPUPERIOD_SEL` writer - CPU clk sel option"]
pub type CPUPERIOD_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CPU_PERIOD_CONF_SPEC, 2, O>;
impl R {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn cpusel_conf(&self) -> CPUSEL_CONF_R {
        CPUSEL_CONF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - CPU clk sel option"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERIOD_CONF")
            .field("cpusel_conf", &format_args!("{}", self.cpusel_conf().bit()))
            .field(
                "cpuperiod_sel",
                &format_args!("{}", self.cpuperiod_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERIOD_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    #[must_use]
    pub fn cpusel_conf(&mut self) -> CPUSEL_CONF_W<29> {
        CPUSEL_CONF_W::new(self)
    }
    #[doc = "Bits 30:31 - CPU clk sel option"]
    #[inline(always)]
    #[must_use]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W<30> {
        CPUPERIOD_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_period_conf](index.html) module"]
pub struct CPU_PERIOD_CONF_SPEC;
impl crate::RegisterSpec for CPU_PERIOD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_period_conf::R](R) reader structure"]
impl crate::Readable for CPU_PERIOD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_period_conf::W](W) writer structure"]
impl crate::Writable for CPU_PERIOD_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PERIOD_CONF to value 0"]
impl crate::Resettable for CPU_PERIOD_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
