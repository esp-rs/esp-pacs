#[doc = "Register `TARGET0_CONF` reader"]
pub type R = crate::R<TARGET0_CONF_SPEC>;
#[doc = "Register `TARGET0_CONF` writer"]
pub type W = crate::W<TARGET0_CONF_SPEC>;
#[doc = "Field `TARGET0_PERIOD` reader - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
pub type TARGET0_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET0_PERIOD` writer - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
pub type TARGET0_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `TARGET0_PERIOD_MODE` reader - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET0_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET0_PERIOD_MODE` writer - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type TARGET0_PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET0_WORK_EN` reader - System timer target 0 work enable."]
pub type TARGET0_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET0_WORK_EN` writer - System timer target 0 work enable."]
pub type TARGET0_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn target0_period(&self) -> TARGET0_PERIOD_R {
        TARGET0_PERIOD_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn target0_period_mode(&self) -> TARGET0_PERIOD_MODE_R {
        TARGET0_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System timer target 0 work enable."]
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET0_WORK_EN_R {
        TARGET0_WORK_EN_R::new(((self.bits >> 31) & 1) != 0)
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
                "target0_work_en",
                &format_args!("{}", self.target0_work_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target0_period(&mut self) -> TARGET0_PERIOD_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn target0_period_mode(&mut self) -> TARGET0_PERIOD_MODE_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - System timer target 0 work enable."]
    #[inline(always)]
    #[must_use]
    pub fn target0_work_en(&mut self) -> TARGET0_WORK_EN_W<TARGET0_CONF_SPEC> {
        TARGET0_WORK_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure work mode for system timer target 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET0_CONF_SPEC;
impl crate::RegisterSpec for TARGET0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target0_conf::R`](R) reader structure"]
impl crate::Readable for TARGET0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target0_conf::W`](W) writer structure"]
impl crate::Writable for TARGET0_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET0_CONF to value 0"]
impl crate::Resettable for TARGET0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
