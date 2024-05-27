///Register `TSENS_CLK_CONF` reader
pub type R = crate::R<TSENS_CLK_CONF_SPEC>;
///Register `TSENS_CLK_CONF` writer
pub type W = crate::W<TSENS_CLK_CONF_SPEC>;
///Field `TSENS_CLK_SEL` reader - set this field to select clock-source. 0(default): FOSC, 1: XTAL.
pub type TSENS_CLK_SEL_R = crate::BitReader;
///Field `TSENS_CLK_SEL` writer - set this field to select clock-source. 0(default): FOSC, 1: XTAL.
pub type TSENS_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_CLK_EN` reader - Set 1 to enable tsens clock
pub type TSENS_CLK_EN_R = crate::BitReader;
///Field `TSENS_CLK_EN` writer - Set 1 to enable tsens clock
pub type TSENS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS_RST_EN` reader - Set 0 to reset tsens module
pub type TSENS_RST_EN_R = crate::BitReader;
///Field `TSENS_RST_EN` writer - Set 0 to reset tsens module
pub type TSENS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 20 - set this field to select clock-source. 0(default): FOSC, 1: XTAL.
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Set 1 to enable tsens clock
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Set 0 to reset tsens module
    #[inline(always)]
    pub fn tsens_rst_en(&self) -> TSENS_RST_EN_R {
        TSENS_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSENS_CLK_CONF")
            .field("tsens_clk_sel", &self.tsens_clk_sel())
            .field("tsens_clk_en", &self.tsens_clk_en())
            .field("tsens_rst_en", &self.tsens_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 20 - set this field to select clock-source. 0(default): FOSC, 1: XTAL.
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W<TSENS_CLK_CONF_SPEC> {
        TSENS_CLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set 1 to enable tsens clock
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W<TSENS_CLK_CONF_SPEC> {
        TSENS_CLK_EN_W::new(self, 22)
    }
    ///Bit 23 - Set 0 to reset tsens module
    #[inline(always)]
    #[must_use]
    pub fn tsens_rst_en(&mut self) -> TSENS_RST_EN_W<TSENS_CLK_CONF_SPEC> {
        TSENS_RST_EN_W::new(self, 23)
    }
}
/**TSENS_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tsens_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsens_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSENS_CLK_CONF_SPEC;
impl crate::RegisterSpec for TSENS_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tsens_clk_conf::R`](R) reader structure
impl crate::Readable for TSENS_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`tsens_clk_conf::W`](W) writer structure
impl crate::Writable for TSENS_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSENS_CLK_CONF to value 0x0040_0000
impl crate::Resettable for TSENS_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
