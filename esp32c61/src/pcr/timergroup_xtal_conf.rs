#[doc = "Register `TIMERGROUP_XTAL_CONF` reader"]
pub type R = crate::R<TIMERGROUP_XTAL_CONF_SPEC>;
#[doc = "Register `TIMERGROUP_XTAL_CONF` writer"]
pub type W = crate::W<TIMERGROUP_XTAL_CONF_SPEC>;
#[doc = "Field `TG0_XTAL_RST_EN` reader - Set 1 to reset timer_group0 xtal clock domain"]
pub type TG0_XTAL_RST_EN_R = crate::BitReader;
#[doc = "Field `TG0_XTAL_RST_EN` writer - Set 1 to reset timer_group0 xtal clock domain"]
pub type TG0_XTAL_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG1_XTAL_RST_EN` reader - Set 1 to reset timer_group1 xtal clock domain"]
pub type TG1_XTAL_RST_EN_R = crate::BitReader;
#[doc = "Field `TG1_XTAL_RST_EN` writer - Set 1 to reset timer_group1 xtal clock domain"]
pub type TG1_XTAL_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_XTAL_CLK_EN` reader - Set 1 to enable tg0 xtal clock"]
pub type TG0_XTAL_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG0_XTAL_CLK_EN` writer - Set 1 to enable tg0 xtal clock"]
pub type TG0_XTAL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to reset timer_group0 xtal clock domain"]
    #[inline(always)]
    pub fn tg0_xtal_rst_en(&self) -> TG0_XTAL_RST_EN_R {
        TG0_XTAL_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset timer_group1 xtal clock domain"]
    #[inline(always)]
    pub fn tg1_xtal_rst_en(&self) -> TG1_XTAL_RST_EN_R {
        TG1_XTAL_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable tg0 xtal clock"]
    #[inline(always)]
    pub fn tg0_xtal_clk_en(&self) -> TG0_XTAL_CLK_EN_R {
        TG0_XTAL_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP_XTAL_CONF")
            .field("tg0_xtal_rst_en", &self.tg0_xtal_rst_en())
            .field("tg1_xtal_rst_en", &self.tg1_xtal_rst_en())
            .field("tg0_xtal_clk_en", &self.tg0_xtal_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to reset timer_group0 xtal clock domain"]
    #[inline(always)]
    pub fn tg0_xtal_rst_en(&mut self) -> TG0_XTAL_RST_EN_W<TIMERGROUP_XTAL_CONF_SPEC> {
        TG0_XTAL_RST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset timer_group1 xtal clock domain"]
    #[inline(always)]
    pub fn tg1_xtal_rst_en(&mut self) -> TG1_XTAL_RST_EN_W<TIMERGROUP_XTAL_CONF_SPEC> {
        TG1_XTAL_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable tg0 xtal clock"]
    #[inline(always)]
    pub fn tg0_xtal_clk_en(&mut self) -> TG0_XTAL_CLK_EN_W<TIMERGROUP_XTAL_CONF_SPEC> {
        TG0_XTAL_CLK_EN_W::new(self, 2)
    }
}
#[doc = "TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup_xtal_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup_xtal_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP_XTAL_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP_XTAL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup_xtal_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP_XTAL_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup_xtal_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP_XTAL_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGROUP_XTAL_CONF to value 0x04"]
impl crate::Resettable for TIMERGROUP_XTAL_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
