#[doc = "Register `SAR_PERI_CLK_GATE_CONF` reader"]
pub type R = crate::R<SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "Register `SAR_PERI_CLK_GATE_CONF` writer"]
pub type W = crate::W<SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "Field `RTC_I2C_CLK_EN` reader - enable rtc i2c clock"]
pub type RTC_I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `RTC_I2C_CLK_EN` writer - enable rtc i2c clock"]
pub type RTC_I2C_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_EN` reader - enable tsens clock"]
pub type TSENS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_EN` writer - enable tsens clock"]
pub type TSENS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_CLK_EN` reader - enbale saradc clock"]
pub type SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `SARADC_CLK_EN` writer - enbale saradc clock"]
pub type SARADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_CLK_EN` reader - enable io_mux clock"]
pub type IOMUX_CLK_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_CLK_EN` writer - enable io_mux clock"]
pub type IOMUX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - enable rtc i2c clock"]
    #[inline(always)]
    pub fn rtc_i2c_clk_en(&self) -> RTC_I2C_CLK_EN_R {
        RTC_I2C_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - enable tsens clock"]
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enbale saradc clock"]
    #[inline(always)]
    pub fn saradc_clk_en(&self) -> SARADC_CLK_EN_R {
        SARADC_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable io_mux clock"]
    #[inline(always)]
    pub fn iomux_clk_en(&self) -> IOMUX_CLK_EN_R {
        IOMUX_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PERI_CLK_GATE_CONF")
            .field("rtc_i2c_clk_en", &self.rtc_i2c_clk_en().bit())
            .field("tsens_clk_en", &self.tsens_clk_en().bit())
            .field("saradc_clk_en", &self.saradc_clk_en().bit())
            .field("iomux_clk_en", &self.iomux_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_PERI_CLK_GATE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 27 - enable rtc i2c clock"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_i2c_clk_en(&mut self) -> RTC_I2C_CLK_EN_W<SAR_PERI_CLK_GATE_CONF_SPEC> {
        RTC_I2C_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 29 - enable tsens clock"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W<SAR_PERI_CLK_GATE_CONF_SPEC> {
        TSENS_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - enbale saradc clock"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_clk_en(&mut self) -> SARADC_CLK_EN_W<SAR_PERI_CLK_GATE_CONF_SPEC> {
        SARADC_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable io_mux clock"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_en(&mut self) -> IOMUX_CLK_EN_W<SAR_PERI_CLK_GATE_CONF_SPEC> {
        IOMUX_CLK_EN_W::new(self, 31)
    }
}
#[doc = "the peri clock gate of rtc peri\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_peri_clk_gate_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_peri_clk_gate_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PERI_CLK_GATE_CONF_SPEC;
impl crate::RegisterSpec for SAR_PERI_CLK_GATE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_peri_clk_gate_conf::R`](R) reader structure"]
impl crate::Readable for SAR_PERI_CLK_GATE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_peri_clk_gate_conf::W`](W) writer structure"]
impl crate::Writable for SAR_PERI_CLK_GATE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_PERI_CLK_GATE_CONF to value 0"]
impl crate::Resettable for SAR_PERI_CLK_GATE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
