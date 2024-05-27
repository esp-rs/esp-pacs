///Register `TWAI0_FUNC_CLK_CONF` reader
pub type R = crate::R<TWAI0_FUNC_CLK_CONF_SPEC>;
///Register `TWAI0_FUNC_CLK_CONF` writer
pub type W = crate::W<TWAI0_FUNC_CLK_CONF_SPEC>;
///Field `TWAI0_FUNC_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: FOSC.
pub type TWAI0_FUNC_CLK_SEL_R = crate::BitReader;
///Field `TWAI0_FUNC_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: FOSC.
pub type TWAI0_FUNC_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWAI0_FUNC_CLK_EN` reader - Set 1 to enable twai0 function clock
pub type TWAI0_FUNC_CLK_EN_R = crate::BitReader;
///Field `TWAI0_FUNC_CLK_EN` writer - Set 1 to enable twai0 function clock
pub type TWAI0_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC.
    #[inline(always)]
    pub fn twai0_func_clk_sel(&self) -> TWAI0_FUNC_CLK_SEL_R {
        TWAI0_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Set 1 to enable twai0 function clock
    #[inline(always)]
    pub fn twai0_func_clk_en(&self) -> TWAI0_FUNC_CLK_EN_R {
        TWAI0_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0_FUNC_CLK_CONF")
            .field("twai0_func_clk_sel", &self.twai0_func_clk_sel())
            .field("twai0_func_clk_en", &self.twai0_func_clk_en())
            .finish()
    }
}
impl W {
    ///Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC.
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_sel(&mut self) -> TWAI0_FUNC_CLK_SEL_W<TWAI0_FUNC_CLK_CONF_SPEC> {
        TWAI0_FUNC_CLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set 1 to enable twai0 function clock
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_en(&mut self) -> TWAI0_FUNC_CLK_EN_W<TWAI0_FUNC_CLK_CONF_SPEC> {
        TWAI0_FUNC_CLK_EN_W::new(self, 22)
    }
}
/**TWAI0_FUNC_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`twai0_func_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai0_func_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TWAI0_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for TWAI0_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`twai0_func_clk_conf::R`](R) reader structure
impl crate::Readable for TWAI0_FUNC_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`twai0_func_clk_conf::W`](W) writer structure
impl crate::Writable for TWAI0_FUNC_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TWAI0_FUNC_CLK_CONF to value 0x0040_0000
impl crate::Resettable for TWAI0_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
