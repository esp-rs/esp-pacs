#[doc = "Register `SAR_IO_MUX_CONF` reader"]
pub type R = crate::R<SAR_IO_MUX_CONF_SPEC>;
#[doc = "Register `SAR_IO_MUX_CONF` writer"]
pub type W = crate::W<SAR_IO_MUX_CONF_SPEC>;
#[doc = "Field `IOMUX_RESET` reader - Reset IO MUX by software"]
pub type IOMUX_RESET_R = crate::BitReader;
#[doc = "Field `IOMUX_RESET` writer - Reset IO MUX by software"]
pub type IOMUX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_CLK_GATE_EN` reader - IO MUX clock gate enable bit"]
pub type IOMUX_CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_CLK_GATE_EN` writer - IO MUX clock gate enable bit"]
pub type IOMUX_CLK_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Reset IO MUX by software"]
    #[inline(always)]
    pub fn iomux_reset(&self) -> IOMUX_RESET_R {
        IOMUX_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IO MUX clock gate enable bit"]
    #[inline(always)]
    pub fn iomux_clk_gate_en(&self) -> IOMUX_CLK_GATE_EN_R {
        IOMUX_CLK_GATE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_IO_MUX_CONF")
            .field("iomux_reset", &format_args!("{}", self.iomux_reset().bit()))
            .field(
                "iomux_clk_gate_en",
                &format_args!("{}", self.iomux_clk_gate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_IO_MUX_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 30 - Reset IO MUX by software"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_reset(&mut self) -> IOMUX_RESET_W<SAR_IO_MUX_CONF_SPEC> {
        IOMUX_RESET_W::new(self, 30)
    }
    #[doc = "Bit 31 - IO MUX clock gate enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_gate_en(&mut self) -> IOMUX_CLK_GATE_EN_W<SAR_IO_MUX_CONF_SPEC> {
        IOMUX_CLK_GATE_EN_W::new(self, 31)
    }
}
#[doc = "Configure and reset IO MUX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_io_mux_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_io_mux_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_IO_MUX_CONF_SPEC;
impl crate::RegisterSpec for SAR_IO_MUX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_io_mux_conf::R`](R) reader structure"]
impl crate::Readable for SAR_IO_MUX_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_io_mux_conf::W`](W) writer structure"]
impl crate::Writable for SAR_IO_MUX_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_IO_MUX_CONF to value 0"]
impl crate::Resettable for SAR_IO_MUX_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
