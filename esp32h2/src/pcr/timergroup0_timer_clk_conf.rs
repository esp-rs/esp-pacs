#[doc = "Register `TIMERGROUP0_TIMER_CLK_CONF` reader"]
pub type R = crate::R<TIMERGROUP0_TIMER_CLK_CONF_SPEC>;
#[doc = "Register `TIMERGROUP0_TIMER_CLK_CONF` writer"]
pub type W = crate::W<TIMERGROUP0_TIMER_CLK_CONF_SPEC>;
#[doc = "Field `TG0_TIMER_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG0_TIMER_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TG0_TIMER_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG0_TIMER_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TG0_TIMER_CLK_EN` reader - Set 1 to enable timer_group0 timer clock"]
pub type TG0_TIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG0_TIMER_CLK_EN` writer - Set 1 to enable timer_group0 timer clock"]
pub type TG0_TIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    pub fn tg0_timer_clk_sel(&self) -> TG0_TIMER_CLK_SEL_R {
        TG0_TIMER_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 timer clock"]
    #[inline(always)]
    pub fn tg0_timer_clk_en(&self) -> TG0_TIMER_CLK_EN_R {
        TG0_TIMER_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP0_TIMER_CLK_CONF")
            .field("tg0_timer_clk_sel", &self.tg0_timer_clk_sel())
            .field("tg0_timer_clk_en", &self.tg0_timer_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tg0_timer_clk_sel(&mut self) -> TG0_TIMER_CLK_SEL_W<TIMERGROUP0_TIMER_CLK_CONF_SPEC> {
        TG0_TIMER_CLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 timer clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_timer_clk_en(&mut self) -> TG0_TIMER_CLK_EN_W<TIMERGROUP0_TIMER_CLK_CONF_SPEC> {
        TG0_TIMER_CLK_EN_W::new(self, 22)
    }
}
#[doc = "TIMERGROUP0_TIMER_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_timer_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_timer_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP0_TIMER_CLK_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP0_TIMER_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup0_timer_clk_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP0_TIMER_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup0_timer_clk_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP0_TIMER_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERGROUP0_TIMER_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TIMERGROUP0_TIMER_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
