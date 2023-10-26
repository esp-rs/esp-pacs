#[doc = "Register `RESET_EN` reader"]
pub type R = crate::R<RESET_EN_SPEC>;
#[doc = "Register `RESET_EN` writer"]
pub type W = crate::W<RESET_EN_SPEC>;
#[doc = "Field `BUS_RESET_EN` writer - need_des"]
pub type BUS_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_BLE_TIMER_RESET_EN` reader - need_des"]
pub type LP_BLE_TIMER_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_BLE_TIMER_RESET_EN` writer - need_des"]
pub type LP_BLE_TIMER_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTP_DBG_RESET_EN` reader - need_des"]
pub type OTP_DBG_RESET_EN_R = crate::BitReader;
#[doc = "Field `OTP_DBG_RESET_EN` writer - need_des"]
pub type OTP_DBG_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_UART_RESET_EN` reader - need_des"]
pub type LP_UART_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_RESET_EN` writer - need_des"]
pub type LP_UART_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_IO_RESET_EN` reader - need_des"]
pub type LP_IO_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_IO_RESET_EN` writer - need_des"]
pub type LP_IO_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_EXT_I2C_RESET_EN` reader - need_des"]
pub type LP_EXT_I2C_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_EXT_I2C_RESET_EN` writer - need_des"]
pub type LP_EXT_I2C_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_ANA_I2C_RESET_EN` reader - need_des"]
pub type LP_ANA_I2C_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_I2C_RESET_EN` writer - need_des"]
pub type LP_ANA_I2C_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFUSE_RESET_EN` reader - need_des"]
pub type EFUSE_RESET_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_RESET_EN` writer - need_des"]
pub type EFUSE_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_CPU_RESET_EN` writer - need_des"]
pub type LP_CPU_RESET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_ble_timer_reset_en(&self) -> LP_BLE_TIMER_RESET_EN_R {
        LP_BLE_TIMER_RESET_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn otp_dbg_reset_en(&self) -> OTP_DBG_RESET_EN_R {
        OTP_DBG_RESET_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_uart_reset_en(&self) -> LP_UART_RESET_EN_R {
        LP_UART_RESET_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_io_reset_en(&self) -> LP_IO_RESET_EN_R {
        LP_IO_RESET_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_ext_i2c_reset_en(&self) -> LP_EXT_I2C_RESET_EN_R {
        LP_EXT_I2C_RESET_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_i2c_reset_en(&self) -> LP_ANA_I2C_RESET_EN_R {
        LP_ANA_I2C_RESET_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn efuse_reset_en(&self) -> EFUSE_RESET_EN_R {
        EFUSE_RESET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_EN")
            .field(
                "lp_ble_timer_reset_en",
                &format_args!("{}", self.lp_ble_timer_reset_en().bit()),
            )
            .field(
                "otp_dbg_reset_en",
                &format_args!("{}", self.otp_dbg_reset_en().bit()),
            )
            .field(
                "lp_uart_reset_en",
                &format_args!("{}", self.lp_uart_reset_en().bit()),
            )
            .field(
                "lp_io_reset_en",
                &format_args!("{}", self.lp_io_reset_en().bit()),
            )
            .field(
                "lp_ext_i2c_reset_en",
                &format_args!("{}", self.lp_ext_i2c_reset_en().bit()),
            )
            .field(
                "lp_ana_i2c_reset_en",
                &format_args!("{}", self.lp_ana_i2c_reset_en().bit()),
            )
            .field(
                "efuse_reset_en",
                &format_args!("{}", self.efuse_reset_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_en(&mut self) -> BUS_RESET_EN_W<RESET_EN_SPEC, 23> {
        BUS_RESET_EN_W::new(self)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ble_timer_reset_en(&mut self) -> LP_BLE_TIMER_RESET_EN_W<RESET_EN_SPEC, 24> {
        LP_BLE_TIMER_RESET_EN_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn otp_dbg_reset_en(&mut self) -> OTP_DBG_RESET_EN_W<RESET_EN_SPEC, 25> {
        OTP_DBG_RESET_EN_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_reset_en(&mut self) -> LP_UART_RESET_EN_W<RESET_EN_SPEC, 26> {
        LP_UART_RESET_EN_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_io_reset_en(&mut self) -> LP_IO_RESET_EN_W<RESET_EN_SPEC, 27> {
        LP_IO_RESET_EN_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ext_i2c_reset_en(&mut self) -> LP_EXT_I2C_RESET_EN_W<RESET_EN_SPEC, 28> {
        LP_EXT_I2C_RESET_EN_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_i2c_reset_en(&mut self) -> LP_ANA_I2C_RESET_EN_W<RESET_EN_SPEC, 29> {
        LP_ANA_I2C_RESET_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_reset_en(&mut self) -> EFUSE_RESET_EN_W<RESET_EN_SPEC, 30> {
        EFUSE_RESET_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_reset_en(&mut self) -> LP_CPU_RESET_EN_W<RESET_EN_SPEC, 31> {
        LP_CPU_RESET_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_EN_SPEC;
impl crate::RegisterSpec for RESET_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_en::R`](R) reader structure"]
impl crate::Readable for RESET_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_en::W`](W) writer structure"]
impl crate::Writable for RESET_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_EN to value 0"]
impl crate::Resettable for RESET_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
