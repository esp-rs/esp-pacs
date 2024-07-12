#[doc = "Register `SYSTIMER_CONF` reader"]
pub type R = crate::R<SYSTIMER_CONF_SPEC>;
#[doc = "Register `SYSTIMER_CONF` writer"]
pub type W = crate::W<SYSTIMER_CONF_SPEC>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Set 1 to enable systimer apb clock"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Set 1 to enable systimer apb clock"]
pub type SYSTIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_RST_EN` reader - Set 0 to reset systimer module"]
pub type SYSTIMER_RST_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST_EN` writer - Set 0 to reset systimer module"]
pub type SYSTIMER_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_READY` reader - Query this field after reset systimer module"]
pub type SYSTIMER_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable systimer apb clock"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset systimer module"]
    #[inline(always)]
    pub fn systimer_rst_en(&self) -> SYSTIMER_RST_EN_R {
        SYSTIMER_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset systimer module"]
    #[inline(always)]
    pub fn systimer_ready(&self) -> SYSTIMER_READY_R {
        SYSTIMER_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_CONF")
            .field("systimer_clk_en", &self.systimer_clk_en())
            .field("systimer_rst_en", &self.systimer_rst_en())
            .field("systimer_ready", &self.systimer_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable systimer apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<SYSTIMER_CONF_SPEC> {
        SYSTIMER_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset systimer module"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_rst_en(&mut self) -> SYSTIMER_RST_EN_W<SYSTIMER_CONF_SPEC> {
        SYSTIMER_RST_EN_W::new(self, 1)
    }
}
#[doc = "SYSTIMER configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_CONF_SPEC;
impl crate::RegisterSpec for SYSTIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_conf::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_conf::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTIMER_CONF to value 0x05"]
impl crate::Resettable for SYSTIMER_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
