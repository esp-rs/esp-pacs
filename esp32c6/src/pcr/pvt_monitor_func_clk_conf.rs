///Register `PVT_MONITOR_FUNC_CLK_CONF` reader
pub type R = crate::R<PVT_MONITOR_FUNC_CLK_CONF_SPEC>;
///Register `PVT_MONITOR_FUNC_CLK_CONF` writer
pub type W = crate::W<PVT_MONITOR_FUNC_CLK_CONF_SPEC>;
///Field `PVT_MONITOR_FUNC_CLK_DIV_NUM` reader - The integral part of the frequency divider factor of the pvt_monitor function clock.
pub type PVT_MONITOR_FUNC_CLK_DIV_NUM_R = crate::FieldReader;
///Field `PVT_MONITOR_FUNC_CLK_DIV_NUM` writer - The integral part of the frequency divider factor of the pvt_monitor function clock.
pub type PVT_MONITOR_FUNC_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PVT_MONITOR_FUNC_CLK_SEL` reader - set this field to select clock-source. 0: XTAL, 1(default): 160MHz drived by SPLL divided by 3.
pub type PVT_MONITOR_FUNC_CLK_SEL_R = crate::BitReader;
///Field `PVT_MONITOR_FUNC_CLK_SEL` writer - set this field to select clock-source. 0: XTAL, 1(default): 160MHz drived by SPLL divided by 3.
pub type PVT_MONITOR_FUNC_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVT_MONITOR_FUNC_CLK_EN` reader - Set 1 to enable source clock of pvt sitex
pub type PVT_MONITOR_FUNC_CLK_EN_R = crate::BitReader;
///Field `PVT_MONITOR_FUNC_CLK_EN` writer - Set 1 to enable source clock of pvt sitex
pub type PVT_MONITOR_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - The integral part of the frequency divider factor of the pvt_monitor function clock.
    #[inline(always)]
    pub fn pvt_monitor_func_clk_div_num(&self) -> PVT_MONITOR_FUNC_CLK_DIV_NUM_R {
        PVT_MONITOR_FUNC_CLK_DIV_NUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 20 - set this field to select clock-source. 0: XTAL, 1(default): 160MHz drived by SPLL divided by 3.
    #[inline(always)]
    pub fn pvt_monitor_func_clk_sel(&self) -> PVT_MONITOR_FUNC_CLK_SEL_R {
        PVT_MONITOR_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Set 1 to enable source clock of pvt sitex
    #[inline(always)]
    pub fn pvt_monitor_func_clk_en(&self) -> PVT_MONITOR_FUNC_CLK_EN_R {
        PVT_MONITOR_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_FUNC_CLK_CONF")
            .field(
                "pvt_monitor_func_clk_div_num",
                &self.pvt_monitor_func_clk_div_num(),
            )
            .field("pvt_monitor_func_clk_sel", &self.pvt_monitor_func_clk_sel())
            .field("pvt_monitor_func_clk_en", &self.pvt_monitor_func_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - The integral part of the frequency divider factor of the pvt_monitor function clock.
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_func_clk_div_num(
        &mut self,
    ) -> PVT_MONITOR_FUNC_CLK_DIV_NUM_W<PVT_MONITOR_FUNC_CLK_CONF_SPEC> {
        PVT_MONITOR_FUNC_CLK_DIV_NUM_W::new(self, 0)
    }
    ///Bit 20 - set this field to select clock-source. 0: XTAL, 1(default): 160MHz drived by SPLL divided by 3.
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_func_clk_sel(
        &mut self,
    ) -> PVT_MONITOR_FUNC_CLK_SEL_W<PVT_MONITOR_FUNC_CLK_CONF_SPEC> {
        PVT_MONITOR_FUNC_CLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set 1 to enable source clock of pvt sitex
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_func_clk_en(
        &mut self,
    ) -> PVT_MONITOR_FUNC_CLK_EN_W<PVT_MONITOR_FUNC_CLK_CONF_SPEC> {
        PVT_MONITOR_FUNC_CLK_EN_W::new(self, 22)
    }
}
/**PVT_MONITOR function clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pvt_monitor_func_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvt_monitor_func_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PVT_MONITOR_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for PVT_MONITOR_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pvt_monitor_func_clk_conf::R`](R) reader structure
impl crate::Readable for PVT_MONITOR_FUNC_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`pvt_monitor_func_clk_conf::W`](W) writer structure
impl crate::Writable for PVT_MONITOR_FUNC_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PVT_MONITOR_FUNC_CLK_CONF to value 0x0040_0000
impl crate::Resettable for PVT_MONITOR_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
