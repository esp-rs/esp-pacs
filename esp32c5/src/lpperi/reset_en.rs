#[doc = "Register `RESET_EN` reader"]
pub type R = crate::R<RESET_EN_SPEC>;
#[doc = "Register `RESET_EN` writer"]
pub type W = crate::W<RESET_EN_SPEC>;
#[doc = "Field `BUS_RESET_EN` writer - lp bus reset enable 1: enable reset 0: disable reset"]
pub type BUS_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RNG_RESET_EN` reader - lp rng reset enable 1: enable reset 0: disable reset"]
pub type LP_RNG_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_RNG_RESET_EN` writer - lp rng reset enable 1: enable reset 0: disable reset"]
pub type LP_RNG_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTP_DBG_RESET_EN` reader - lp optdebug reset enable 1: enable reset 0: disable reset"]
pub type OTP_DBG_RESET_EN_R = crate::BitReader;
#[doc = "Field `OTP_DBG_RESET_EN` writer - lp optdebug reset enable 1: enable reset 0: disable reset"]
pub type OTP_DBG_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_RESET_EN` reader - lp uart reset enable 1: enable reset 0: disable reset"]
pub type LP_UART_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_RESET_EN` writer - lp uart reset enable 1: enable reset 0: disable reset"]
pub type LP_UART_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_IO_RESET_EN` reader - lp io reset enable 1: enable reset 0: disable reset"]
pub type LP_IO_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_IO_RESET_EN` writer - lp io reset enable 1: enable reset 0: disable reset"]
pub type LP_IO_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_EXT_I2C_RESET_EN` reader - lp ext i2c reset enable 1: enable reset 0: disable reset"]
pub type LP_EXT_I2C_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_EXT_I2C_RESET_EN` writer - lp ext i2c reset enable 1: enable reset 0: disable reset"]
pub type LP_EXT_I2C_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_I2C_RESET_EN` reader - lp analog peri reset enable 1: enable reset 0: disable reset"]
pub type LP_ANA_I2C_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_I2C_RESET_EN` writer - lp analog peri reset enable 1: enable reset 0: disable reset"]
pub type LP_ANA_I2C_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_RESET_EN` reader - efuse core reset enable 1: enable reset 0: disable reset"]
pub type EFUSE_RESET_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_RESET_EN` writer - efuse core reset enable 1: enable reset 0: disable reset"]
pub type EFUSE_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CPU_RESET_EN` writer - force on lp cpu reset enable 1: enable cpu reset 0: cpu reset is controlled by pmu"]
pub type LP_CPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - lp rng reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_rng_reset_en(&self) -> LP_RNG_RESET_EN_R {
        LP_RNG_RESET_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - lp optdebug reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn otp_dbg_reset_en(&self) -> OTP_DBG_RESET_EN_R {
        OTP_DBG_RESET_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - lp uart reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_uart_reset_en(&self) -> LP_UART_RESET_EN_R {
        LP_UART_RESET_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - lp io reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_io_reset_en(&self) -> LP_IO_RESET_EN_R {
        LP_IO_RESET_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - lp ext i2c reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_ext_i2c_reset_en(&self) -> LP_EXT_I2C_RESET_EN_R {
        LP_EXT_I2C_RESET_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - lp analog peri reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_ana_i2c_reset_en(&self) -> LP_ANA_I2C_RESET_EN_R {
        LP_ANA_I2C_RESET_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - efuse core reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn efuse_reset_en(&self) -> EFUSE_RESET_EN_R {
        EFUSE_RESET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_EN")
            .field("lp_rng_reset_en", &self.lp_rng_reset_en())
            .field("otp_dbg_reset_en", &self.otp_dbg_reset_en())
            .field("lp_uart_reset_en", &self.lp_uart_reset_en())
            .field("lp_io_reset_en", &self.lp_io_reset_en())
            .field("lp_ext_i2c_reset_en", &self.lp_ext_i2c_reset_en())
            .field("lp_ana_i2c_reset_en", &self.lp_ana_i2c_reset_en())
            .field("efuse_reset_en", &self.efuse_reset_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - lp bus reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn bus_reset_en(&mut self) -> BUS_RESET_EN_W<RESET_EN_SPEC> {
        BUS_RESET_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - lp rng reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_rng_reset_en(&mut self) -> LP_RNG_RESET_EN_W<RESET_EN_SPEC> {
        LP_RNG_RESET_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - lp optdebug reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn otp_dbg_reset_en(&mut self) -> OTP_DBG_RESET_EN_W<RESET_EN_SPEC> {
        OTP_DBG_RESET_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - lp uart reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_uart_reset_en(&mut self) -> LP_UART_RESET_EN_W<RESET_EN_SPEC> {
        LP_UART_RESET_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - lp io reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_io_reset_en(&mut self) -> LP_IO_RESET_EN_W<RESET_EN_SPEC> {
        LP_IO_RESET_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - lp ext i2c reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_ext_i2c_reset_en(&mut self) -> LP_EXT_I2C_RESET_EN_W<RESET_EN_SPEC> {
        LP_EXT_I2C_RESET_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - lp analog peri reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn lp_ana_i2c_reset_en(&mut self) -> LP_ANA_I2C_RESET_EN_W<RESET_EN_SPEC> {
        LP_ANA_I2C_RESET_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - efuse core reset enable 1: enable reset 0: disable reset"]
    #[inline(always)]
    pub fn efuse_reset_en(&mut self) -> EFUSE_RESET_EN_W<RESET_EN_SPEC> {
        EFUSE_RESET_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - force on lp cpu reset enable 1: enable cpu reset 0: cpu reset is controlled by pmu"]
    #[inline(always)]
    pub fn lp_cpu_reset_en(&mut self) -> LP_CPU_RESET_EN_W<RESET_EN_SPEC> {
        LP_CPU_RESET_EN_W::new(self, 31)
    }
}
#[doc = "configure peri in lp system reset enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_EN_SPEC;
impl crate::RegisterSpec for RESET_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_en::R`](R) reader structure"]
impl crate::Readable for RESET_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_en::W`](W) writer structure"]
impl crate::Writable for RESET_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_EN to value 0"]
impl crate::Resettable for RESET_EN_SPEC {}
