#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `DUTY_RES` reader - This register is used to control the range of the counter in timer %s."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - This register is used to control the range of the counter in timer %s."]
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_DIV` reader - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DIV` writer - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PAUSE` reader - This bit is used to suspend the counter in timer %s."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - This bit is used to suspend the counter in timer %s."]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL` reader - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP` writer - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 4) & 0x0003_ffff)
    }
    #[doc = "Bit 22 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("duty_res", &self.duty_res().bits())
            .field("clk_div", &self.clk_div().bits())
            .field("pause", &self.pause().bit())
            .field("rst", &self.rst().bit())
            .field("tick_sel", &self.tick_sel().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 4:21 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<CONF_SPEC> {
        CLK_DIV_W::new(self, 4)
    }
    #[doc = "Bit 22 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<CONF_SPEC> {
        PAUSE_W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CONF_SPEC> {
        RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 0: LEDC_PWM_CLK. 1: REF_TICK."]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<CONF_SPEC> {
        TICK_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<CONF_SPEC> {
        PARA_UP_W::new(self, 25)
    }
}
#[doc = "Timer 0 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0x0080_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
