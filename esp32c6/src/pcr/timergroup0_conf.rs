#[doc = "Register `TIMERGROUP0_CONF` reader"]
pub type R = crate::R<TIMERGROUP0_CONF_SPEC>;
#[doc = "Register `TIMERGROUP0_CONF` writer"]
pub type W = crate::W<TIMERGROUP0_CONF_SPEC>;
#[doc = "Field `TG0_CLK_EN` reader - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG0_CLK_EN` writer - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_RST_EN` reader - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_R = crate::BitReader;
#[doc = "Field `TG0_RST_EN` writer - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn tg0_clk_en(&self) -> TG0_CLK_EN_R {
        TG0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn tg0_rst_en(&self) -> TG0_RST_EN_R {
        TG0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP0_CONF")
            .field("tg0_clk_en", &self.tg0_clk_en())
            .field("tg0_rst_en", &self.tg0_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_clk_en(&mut self) -> TG0_CLK_EN_W<TIMERGROUP0_CONF_SPEC> {
        TG0_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_rst_en(&mut self) -> TG0_RST_EN_W<TIMERGROUP0_CONF_SPEC> {
        TG0_RST_EN_W::new(self, 1)
    }
}
#[doc = "TIMERGROUP0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergroup0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP0_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup0_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup0_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP0_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERGROUP0_CONF to value 0x01"]
impl crate::Resettable for TIMERGROUP0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
