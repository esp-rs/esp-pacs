#[doc = "Register `WR_TIM_CONF1` reader"]
pub type R = crate::R<WR_TIM_CONF1_SPEC>;
#[doc = "Register `WR_TIM_CONF1` writer"]
pub type W = crate::W<WR_TIM_CONF1_SPEC>;
#[doc = "Field `TSUP_A` reader - Configures the setup time of programming operation."]
pub type TSUP_A_R = crate::FieldReader;
#[doc = "Field `TSUP_A` writer - Configures the setup time of programming operation."]
pub type TSUP_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWR_ON_NUM` reader - Configures the power up time for VDDQ."]
pub type PWR_ON_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PWR_ON_NUM` writer - Configures the power up time for VDDQ."]
pub type PWR_ON_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Configures the setup time of programming operation."]
    #[inline(always)]
    pub fn tsup_a(&self) -> TSUP_A_R {
        TSUP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Configures the power up time for VDDQ."]
    #[inline(always)]
    pub fn pwr_on_num(&self) -> PWR_ON_NUM_R {
        PWR_ON_NUM_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF1")
            .field("tsup_a", &self.tsup_a().bits())
            .field("pwr_on_num", &self.pwr_on_num().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the setup time of programming operation."]
    #[inline(always)]
    #[must_use]
    pub fn tsup_a(&mut self) -> TSUP_A_W<WR_TIM_CONF1_SPEC> {
        TSUP_A_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Configures the power up time for VDDQ."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_on_num(&mut self) -> PWR_ON_NUM_W<WR_TIM_CONF1_SPEC> {
        PWR_ON_NUM_W::new(self, 8)
    }
}
#[doc = "Configuration register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_TIM_CONF1_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_tim_conf1::R`](R) reader structure"]
impl crate::Readable for WR_TIM_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_tim_conf1::W`](W) writer structure"]
impl crate::Writable for WR_TIM_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF1 to value 0x0028_8001"]
impl crate::Resettable for WR_TIM_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x0028_8001;
}
