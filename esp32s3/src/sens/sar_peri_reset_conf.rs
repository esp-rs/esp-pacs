#[doc = "Register `SAR_PERI_RESET_CONF` reader"]
pub type R = crate::R<SAR_PERI_RESET_CONF_SPEC>;
#[doc = "Register `SAR_PERI_RESET_CONF` writer"]
pub type W = crate::W<SAR_PERI_RESET_CONF_SPEC>;
#[doc = "Field `SAR_COCPU_RESET` reader - enable ulp-riscv reset"]
pub type SAR_COCPU_RESET_R = crate::BitReader;
#[doc = "Field `SAR_COCPU_RESET` writer - enable ulp-riscv reset"]
pub type SAR_COCPU_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_RTC_I2C_RESET` reader - Reserved."]
pub type SAR_RTC_I2C_RESET_R = crate::BitReader;
#[doc = "Field `SAR_RTC_I2C_RESET` writer - Reserved."]
pub type SAR_RTC_I2C_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_TSENS_RESET` reader - enbale saradc reset"]
pub type SAR_TSENS_RESET_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_RESET` writer - enbale saradc reset"]
pub type SAR_TSENS_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_SARADC_RESET` reader - enable io_mux reset"]
pub type SAR_SARADC_RESET_R = crate::BitReader;
#[doc = "Field `SAR_SARADC_RESET` writer - enable io_mux reset"]
pub type SAR_SARADC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 25 - enable ulp-riscv reset"]
    #[inline(always)]
    pub fn sar_cocpu_reset(&self) -> SAR_COCPU_RESET_R {
        SAR_COCPU_RESET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved."]
    #[inline(always)]
    pub fn sar_rtc_i2c_reset(&self) -> SAR_RTC_I2C_RESET_R {
        SAR_RTC_I2C_RESET_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - enbale saradc reset"]
    #[inline(always)]
    pub fn sar_tsens_reset(&self) -> SAR_TSENS_RESET_R {
        SAR_TSENS_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable io_mux reset"]
    #[inline(always)]
    pub fn sar_saradc_reset(&self) -> SAR_SARADC_RESET_R {
        SAR_SARADC_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PERI_RESET_CONF")
            .field("sar_cocpu_reset", &self.sar_cocpu_reset())
            .field("sar_rtc_i2c_reset", &self.sar_rtc_i2c_reset())
            .field("sar_tsens_reset", &self.sar_tsens_reset())
            .field("sar_saradc_reset", &self.sar_saradc_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 25 - enable ulp-riscv reset"]
    #[inline(always)]
    pub fn sar_cocpu_reset(&mut self) -> SAR_COCPU_RESET_W<SAR_PERI_RESET_CONF_SPEC> {
        SAR_COCPU_RESET_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved."]
    #[inline(always)]
    pub fn sar_rtc_i2c_reset(&mut self) -> SAR_RTC_I2C_RESET_W<SAR_PERI_RESET_CONF_SPEC> {
        SAR_RTC_I2C_RESET_W::new(self, 27)
    }
    #[doc = "Bit 29 - enbale saradc reset"]
    #[inline(always)]
    pub fn sar_tsens_reset(&mut self) -> SAR_TSENS_RESET_W<SAR_PERI_RESET_CONF_SPEC> {
        SAR_TSENS_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - enable io_mux reset"]
    #[inline(always)]
    pub fn sar_saradc_reset(&mut self) -> SAR_SARADC_RESET_W<SAR_PERI_RESET_CONF_SPEC> {
        SAR_SARADC_RESET_W::new(self, 30)
    }
}
#[doc = "the peri reset of rtc peri\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_peri_reset_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_peri_reset_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PERI_RESET_CONF_SPEC;
impl crate::RegisterSpec for SAR_PERI_RESET_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_peri_reset_conf::R`](R) reader structure"]
impl crate::Readable for SAR_PERI_RESET_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_peri_reset_conf::W`](W) writer structure"]
impl crate::Writable for SAR_PERI_RESET_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_PERI_RESET_CONF to value 0"]
impl crate::Resettable for SAR_PERI_RESET_CONF_SPEC {}
