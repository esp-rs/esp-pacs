///Register `REGCLK` reader
pub type R = crate::R<REGCLK_SPEC>;
///Register `REGCLK` writer
pub type W = crate::W<REGCLK_SPEC>;
///Field `WDT_CLK_IS_ACTIVE` reader - enable WDT's clock
pub type WDT_CLK_IS_ACTIVE_R = crate::BitReader;
///Field `WDT_CLK_IS_ACTIVE` writer - enable WDT's clock
pub type WDT_CLK_IS_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_CLK_IS_ACTIVE` reader - enable Timer 30's clock
pub type TIMER_CLK_IS_ACTIVE_R = crate::BitReader;
///Field `TIMER_CLK_IS_ACTIVE` writer - enable Timer 30's clock
pub type TIMER_CLK_IS_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 29 - enable WDT's clock
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> WDT_CLK_IS_ACTIVE_R {
        WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable Timer 30's clock
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> TIMER_CLK_IS_ACTIVE_R {
        TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCLK")
            .field("wdt_clk_is_active", &self.wdt_clk_is_active())
            .field("timer_clk_is_active", &self.timer_clk_is_active())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bit 29 - enable WDT's clock
    #[inline(always)]
    #[must_use]
    pub fn wdt_clk_is_active(&mut self) -> WDT_CLK_IS_ACTIVE_W<REGCLK_SPEC> {
        WDT_CLK_IS_ACTIVE_W::new(self, 29)
    }
    ///Bit 30 - enable Timer 30's clock
    #[inline(always)]
    #[must_use]
    pub fn timer_clk_is_active(&mut self) -> TIMER_CLK_IS_ACTIVE_W<REGCLK_SPEC> {
        TIMER_CLK_IS_ACTIVE_W::new(self, 30)
    }
    ///Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGCLK_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
/**Timer group clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`regclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`regclk::R`](R) reader structure
impl crate::Readable for REGCLK_SPEC {}
///`write(|w| ..)` method takes [`regclk::W`](W) writer structure
impl crate::Writable for REGCLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGCLK to value 0x6000_0000
impl crate::Resettable for REGCLK_SPEC {
    const RESET_VALUE: u32 = 0x6000_0000;
}
