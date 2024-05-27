///Register `SAR_IO_MUX_CONF` reader
pub type R = crate::R<SAR_IO_MUX_CONF_SPEC>;
///Register `SAR_IO_MUX_CONF` writer
pub type W = crate::W<SAR_IO_MUX_CONF_SPEC>;
///Field `IOMUX_RESET` reader - Reset IO MUX by software
pub type IOMUX_RESET_R = crate::BitReader;
///Field `IOMUX_RESET` writer - Reset IO MUX by software
pub type IOMUX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOMUX_CLK_GATE_EN` reader - IO MUX clock gate enable bit
pub type IOMUX_CLK_GATE_EN_R = crate::BitReader;
///Field `IOMUX_CLK_GATE_EN` writer - IO MUX clock gate enable bit
pub type IOMUX_CLK_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - Reset IO MUX by software
    #[inline(always)]
    pub fn iomux_reset(&self) -> IOMUX_RESET_R {
        IOMUX_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - IO MUX clock gate enable bit
    #[inline(always)]
    pub fn iomux_clk_gate_en(&self) -> IOMUX_CLK_GATE_EN_R {
        IOMUX_CLK_GATE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_IO_MUX_CONF")
            .field("iomux_reset", &self.iomux_reset())
            .field("iomux_clk_gate_en", &self.iomux_clk_gate_en())
            .finish()
    }
}
impl W {
    ///Bit 30 - Reset IO MUX by software
    #[inline(always)]
    #[must_use]
    pub fn iomux_reset(&mut self) -> IOMUX_RESET_W<SAR_IO_MUX_CONF_SPEC> {
        IOMUX_RESET_W::new(self, 30)
    }
    ///Bit 31 - IO MUX clock gate enable bit
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_gate_en(&mut self) -> IOMUX_CLK_GATE_EN_W<SAR_IO_MUX_CONF_SPEC> {
        IOMUX_CLK_GATE_EN_W::new(self, 31)
    }
}
/**Configure and reset IO MUX

You can [`read`](crate::generic::Reg::read) this register and get [`sar_io_mux_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_io_mux_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_IO_MUX_CONF_SPEC;
impl crate::RegisterSpec for SAR_IO_MUX_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_io_mux_conf::R`](R) reader structure
impl crate::Readable for SAR_IO_MUX_CONF_SPEC {}
///`write(|w| ..)` method takes [`sar_io_mux_conf::W`](W) writer structure
impl crate::Writable for SAR_IO_MUX_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_IO_MUX_CONF to value 0
impl crate::Resettable for SAR_IO_MUX_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
