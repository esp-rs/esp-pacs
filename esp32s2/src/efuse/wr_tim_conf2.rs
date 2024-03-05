#[doc = "Register `WR_TIM_CONF2` reader"]
pub type R = crate::R<WR_TIM_CONF2_SPEC>;
#[doc = "Register `WR_TIM_CONF2` writer"]
pub type W = crate::W<WR_TIM_CONF2_SPEC>;
#[doc = "Field `PWR_OFF_NUM` reader - Configures the power outage time for VDDQ."]
pub type PWR_OFF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PWR_OFF_NUM` writer - Configures the power outage time for VDDQ."]
pub type PWR_OFF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    pub fn pwr_off_num(&self) -> PWR_OFF_NUM_R {
        PWR_OFF_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF2")
            .field(
                "pwr_off_num",
                &format_args!("{}", self.pwr_off_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_off_num(&mut self) -> PWR_OFF_NUM_W<WR_TIM_CONF2_SPEC> {
        PWR_OFF_NUM_W::new(self, 0)
    }
}
#[doc = "Configuration register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_TIM_CONF2_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_tim_conf2::R`](R) reader structure"]
impl crate::Readable for WR_TIM_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_tim_conf2::W`](W) writer structure"]
impl crate::Writable for WR_TIM_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF2 to value 0x0190"]
impl crate::Resettable for WR_TIM_CONF2_SPEC {
    const RESET_VALUE: u32 = 0x0190;
}
