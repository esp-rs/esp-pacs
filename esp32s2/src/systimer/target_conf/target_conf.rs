#[doc = "Register `TARGET_CONF` reader"]
pub type R = crate::R<TARGET_CONF_SPEC>;
#[doc = "Register `TARGET_CONF` writer"]
pub type W = crate::W<TARGET_CONF_SPEC>;
#[doc = "Field `PERIOD` reader - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `PERIOD_MODE` reader - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `PERIOD_MODE` writer - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
pub type PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_EN` reader - System timer target 0 work enable."]
pub type WORK_EN_R = crate::BitReader;
#[doc = "Field `WORK_EN` writer - System timer target 0 work enable."]
pub type WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    pub fn period_mode(&self) -> PERIOD_MODE_R {
        PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System timer target 0 work enable."]
    #[inline(always)]
    pub fn work_en(&self) -> WORK_EN_R {
        WORK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET_CONF")
            .field("period", &format_args!("{}", self.period().bits()))
            .field("period_mode", &format_args!("{}", self.period_mode().bit()))
            .field("work_en", &format_args!("{}", self.work_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set alarm period for system timer target 0, only valid in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<TARGET_CONF_SPEC> {
        PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Set work mode for system timer target 0. 0: work in a timedelay alarm mode; 1: work in periodic alarms mode."]
    #[inline(always)]
    #[must_use]
    pub fn period_mode(&mut self) -> PERIOD_MODE_W<TARGET_CONF_SPEC> {
        PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - System timer target 0 work enable."]
    #[inline(always)]
    #[must_use]
    pub fn work_en(&mut self) -> WORK_EN_W<TARGET_CONF_SPEC> {
        WORK_EN_W::new(self, 31)
    }
}
#[doc = "Configure work mode for system timer target 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET_CONF_SPEC;
impl crate::RegisterSpec for TARGET_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_conf::R`](R) reader structure"]
impl crate::Readable for TARGET_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target_conf::W`](W) writer structure"]
impl crate::Writable for TARGET_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET_CONF to value 0"]
impl crate::Resettable for TARGET_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
