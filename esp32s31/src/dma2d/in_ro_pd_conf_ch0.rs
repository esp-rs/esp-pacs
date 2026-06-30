#[doc = "Register `IN_RO_PD_CONF_CH0` reader"]
pub type R = crate::R<IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "Register `IN_RO_PD_CONF_CH0` writer"]
pub type W = crate::W<IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "Field `IN_RO_RAM_FORCE_PD_CH` reader - unused! dma reorder ram power down"]
pub type IN_RO_RAM_FORCE_PD_CH_R = crate::BitReader;
#[doc = "Field `IN_RO_RAM_FORCE_PD_CH` writer - unused! dma reorder ram power down"]
pub type IN_RO_RAM_FORCE_PD_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RO_RAM_FORCE_PU_CH` reader - unused! dma reorder ram power up"]
pub type IN_RO_RAM_FORCE_PU_CH_R = crate::BitReader;
#[doc = "Field `IN_RO_RAM_FORCE_PU_CH` writer - unused! dma reorder ram power up"]
pub type IN_RO_RAM_FORCE_PU_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RO_RAM_CLK_FO_CH` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type IN_RO_RAM_CLK_FO_CH_R = crate::BitReader;
#[doc = "Field `IN_RO_RAM_CLK_FO_CH` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type IN_RO_RAM_CLK_FO_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - unused! dma reorder ram power down"]
    #[inline(always)]
    pub fn in_ro_ram_force_pd_ch(&self) -> IN_RO_RAM_FORCE_PD_CH_R {
        IN_RO_RAM_FORCE_PD_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - unused! dma reorder ram power up"]
    #[inline(always)]
    pub fn in_ro_ram_force_pu_ch(&self) -> IN_RO_RAM_FORCE_PU_CH_R {
        IN_RO_RAM_FORCE_PU_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn in_ro_ram_clk_fo_ch(&self) -> IN_RO_RAM_CLK_FO_CH_R {
        IN_RO_RAM_CLK_FO_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_RO_PD_CONF_CH0")
            .field("in_ro_ram_force_pd_ch", &self.in_ro_ram_force_pd_ch())
            .field("in_ro_ram_force_pu_ch", &self.in_ro_ram_force_pu_ch())
            .field("in_ro_ram_clk_fo_ch", &self.in_ro_ram_clk_fo_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - unused! dma reorder ram power down"]
    #[inline(always)]
    pub fn in_ro_ram_force_pd_ch(&mut self) -> IN_RO_RAM_FORCE_PD_CH_W<'_, IN_RO_PD_CONF_CH0_SPEC> {
        IN_RO_RAM_FORCE_PD_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - unused! dma reorder ram power up"]
    #[inline(always)]
    pub fn in_ro_ram_force_pu_ch(&mut self) -> IN_RO_RAM_FORCE_PU_CH_W<'_, IN_RO_PD_CONF_CH0_SPEC> {
        IN_RO_RAM_FORCE_PU_CH_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn in_ro_ram_clk_fo_ch(&mut self) -> IN_RO_RAM_CLK_FO_CH_W<'_, IN_RO_PD_CONF_CH0_SPEC> {
        IN_RO_RAM_CLK_FO_CH_W::new(self, 6)
    }
}
#[doc = "Configures the rx reorder memory of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ro_pd_conf_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ro_pd_conf_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_RO_PD_CONF_CH0_SPEC;
impl crate::RegisterSpec for IN_RO_PD_CONF_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ro_pd_conf_ch0::R`](R) reader structure"]
impl crate::Readable for IN_RO_PD_CONF_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_ro_pd_conf_ch0::W`](W) writer structure"]
impl crate::Writable for IN_RO_PD_CONF_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_RO_PD_CONF_CH0 to value 0x10"]
impl crate::Resettable for IN_RO_PD_CONF_CH0_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
