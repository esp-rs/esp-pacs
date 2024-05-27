///Register `LP_TIMER_CONF` reader
pub type R = crate::R<LP_TIMER_CONF_SPEC>;
///Register `LP_TIMER_CONF` writer
pub type W = crate::W<LP_TIMER_CONF_SPEC>;
///Field `CLK_LP_TIMER_SEL_OSC_SLOW` reader -
pub type CLK_LP_TIMER_SEL_OSC_SLOW_R = crate::BitReader;
///Field `CLK_LP_TIMER_SEL_OSC_SLOW` writer -
pub type CLK_LP_TIMER_SEL_OSC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_LP_TIMER_SEL_OSC_FAST` reader -
pub type CLK_LP_TIMER_SEL_OSC_FAST_R = crate::BitReader;
///Field `CLK_LP_TIMER_SEL_OSC_FAST` writer -
pub type CLK_LP_TIMER_SEL_OSC_FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_LP_TIMER_SEL_XTAL` reader -
pub type CLK_LP_TIMER_SEL_XTAL_R = crate::BitReader;
///Field `CLK_LP_TIMER_SEL_XTAL` writer -
pub type CLK_LP_TIMER_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_LP_TIMER_SEL_XTAL32K` reader -
pub type CLK_LP_TIMER_SEL_XTAL32K_R = crate::BitReader;
///Field `CLK_LP_TIMER_SEL_XTAL32K` writer -
pub type CLK_LP_TIMER_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_LP_TIMER_DIV_NUM` reader -
pub type CLK_LP_TIMER_DIV_NUM_R = crate::FieldReader<u16>;
///Field `CLK_LP_TIMER_DIV_NUM` writer -
pub type CLK_LP_TIMER_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn clk_lp_timer_sel_osc_slow(&self) -> CLK_LP_TIMER_SEL_OSC_SLOW_R {
        CLK_LP_TIMER_SEL_OSC_SLOW_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn clk_lp_timer_sel_osc_fast(&self) -> CLK_LP_TIMER_SEL_OSC_FAST_R {
        CLK_LP_TIMER_SEL_OSC_FAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn clk_lp_timer_sel_xtal(&self) -> CLK_LP_TIMER_SEL_XTAL_R {
        CLK_LP_TIMER_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn clk_lp_timer_sel_xtal32k(&self) -> CLK_LP_TIMER_SEL_XTAL32K_R {
        CLK_LP_TIMER_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:15
    #[inline(always)]
    pub fn clk_lp_timer_div_num(&self) -> CLK_LP_TIMER_DIV_NUM_R {
        CLK_LP_TIMER_DIV_NUM_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TIMER_CONF")
            .field(
                "clk_lp_timer_sel_osc_slow",
                &self.clk_lp_timer_sel_osc_slow(),
            )
            .field(
                "clk_lp_timer_sel_osc_fast",
                &self.clk_lp_timer_sel_osc_fast(),
            )
            .field("clk_lp_timer_sel_xtal", &self.clk_lp_timer_sel_xtal())
            .field("clk_lp_timer_sel_xtal32k", &self.clk_lp_timer_sel_xtal32k())
            .field("clk_lp_timer_div_num", &self.clk_lp_timer_div_num())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_sel_osc_slow(&mut self) -> CLK_LP_TIMER_SEL_OSC_SLOW_W<LP_TIMER_CONF_SPEC> {
        CLK_LP_TIMER_SEL_OSC_SLOW_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_sel_osc_fast(&mut self) -> CLK_LP_TIMER_SEL_OSC_FAST_W<LP_TIMER_CONF_SPEC> {
        CLK_LP_TIMER_SEL_OSC_FAST_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_sel_xtal(&mut self) -> CLK_LP_TIMER_SEL_XTAL_W<LP_TIMER_CONF_SPEC> {
        CLK_LP_TIMER_SEL_XTAL_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_sel_xtal32k(&mut self) -> CLK_LP_TIMER_SEL_XTAL32K_W<LP_TIMER_CONF_SPEC> {
        CLK_LP_TIMER_SEL_XTAL32K_W::new(self, 3)
    }
    ///Bits 4:15
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_div_num(&mut self) -> CLK_LP_TIMER_DIV_NUM_W<LP_TIMER_CONF_SPEC> {
        CLK_LP_TIMER_DIV_NUM_W::new(self, 4)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`lp_timer_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timer_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_TIMER_CONF_SPEC;
impl crate::RegisterSpec for LP_TIMER_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_timer_conf::R`](R) reader structure
impl crate::Readable for LP_TIMER_CONF_SPEC {}
///`write(|w| ..)` method takes [`lp_timer_conf::W`](W) writer structure
impl crate::Writable for LP_TIMER_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_TIMER_CONF to value 0
impl crate::Resettable for LP_TIMER_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
