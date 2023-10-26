#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `LP_TOUCH_CK_EN` reader - need_des"]
pub type LP_TOUCH_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_TOUCH_CK_EN` writer - need_des"]
pub type LP_TOUCH_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RNG_CK_EN` reader - need_des"]
pub type RNG_CK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CK_EN` writer - need_des"]
pub type RNG_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTP_DBG_CK_EN` reader - need_des"]
pub type OTP_DBG_CK_EN_R = crate::BitReader;
#[doc = "Field `OTP_DBG_CK_EN` writer - need_des"]
pub type OTP_DBG_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_UART_CK_EN` reader - need_des"]
pub type LP_UART_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_CK_EN` writer - need_des"]
pub type LP_UART_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_IO_CK_EN` reader - need_des"]
pub type LP_IO_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_IO_CK_EN` writer - need_des"]
pub type LP_IO_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_EXT_I2C_CK_EN` reader - need_des"]
pub type LP_EXT_I2C_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_EXT_I2C_CK_EN` writer - need_des"]
pub type LP_EXT_I2C_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_ANA_I2C_CK_EN` reader - need_des"]
pub type LP_ANA_I2C_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_I2C_CK_EN` writer - need_des"]
pub type LP_ANA_I2C_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFUSE_CK_EN` reader - need_des"]
pub type EFUSE_CK_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_CK_EN` writer - need_des"]
pub type EFUSE_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LP_CPU_CK_EN` reader - need_des"]
pub type LP_CPU_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_CPU_CK_EN` writer - need_des"]
pub type LP_CPU_CK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn lp_touch_ck_en(&self) -> LP_TOUCH_CK_EN_R {
        LP_TOUCH_CK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn rng_ck_en(&self) -> RNG_CK_EN_R {
        RNG_CK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn otp_dbg_ck_en(&self) -> OTP_DBG_CK_EN_R {
        OTP_DBG_CK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_uart_ck_en(&self) -> LP_UART_CK_EN_R {
        LP_UART_CK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_io_ck_en(&self) -> LP_IO_CK_EN_R {
        LP_IO_CK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_ext_i2c_ck_en(&self) -> LP_EXT_I2C_CK_EN_R {
        LP_EXT_I2C_CK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_i2c_ck_en(&self) -> LP_ANA_I2C_CK_EN_R {
        LP_ANA_I2C_CK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn efuse_ck_en(&self) -> EFUSE_CK_EN_R {
        EFUSE_CK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_ck_en(&self) -> LP_CPU_CK_EN_R {
        LP_CPU_CK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field(
                "lp_touch_ck_en",
                &format_args!("{}", self.lp_touch_ck_en().bit()),
            )
            .field("rng_ck_en", &format_args!("{}", self.rng_ck_en().bit()))
            .field(
                "otp_dbg_ck_en",
                &format_args!("{}", self.otp_dbg_ck_en().bit()),
            )
            .field(
                "lp_uart_ck_en",
                &format_args!("{}", self.lp_uart_ck_en().bit()),
            )
            .field("lp_io_ck_en", &format_args!("{}", self.lp_io_ck_en().bit()))
            .field(
                "lp_ext_i2c_ck_en",
                &format_args!("{}", self.lp_ext_i2c_ck_en().bit()),
            )
            .field(
                "lp_ana_i2c_ck_en",
                &format_args!("{}", self.lp_ana_i2c_ck_en().bit()),
            )
            .field("efuse_ck_en", &format_args!("{}", self.efuse_ck_en().bit()))
            .field(
                "lp_cpu_ck_en",
                &format_args!("{}", self.lp_cpu_ck_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_touch_ck_en(&mut self) -> LP_TOUCH_CK_EN_W<CLK_EN_SPEC, 23> {
        LP_TOUCH_CK_EN_W::new(self)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rng_ck_en(&mut self) -> RNG_CK_EN_W<CLK_EN_SPEC, 24> {
        RNG_CK_EN_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn otp_dbg_ck_en(&mut self) -> OTP_DBG_CK_EN_W<CLK_EN_SPEC, 25> {
        OTP_DBG_CK_EN_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_ck_en(&mut self) -> LP_UART_CK_EN_W<CLK_EN_SPEC, 26> {
        LP_UART_CK_EN_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_io_ck_en(&mut self) -> LP_IO_CK_EN_W<CLK_EN_SPEC, 27> {
        LP_IO_CK_EN_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ext_i2c_ck_en(&mut self) -> LP_EXT_I2C_CK_EN_W<CLK_EN_SPEC, 28> {
        LP_EXT_I2C_CK_EN_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_i2c_ck_en(&mut self) -> LP_ANA_I2C_CK_EN_W<CLK_EN_SPEC, 29> {
        LP_ANA_I2C_CK_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_ck_en(&mut self) -> EFUSE_CK_EN_W<CLK_EN_SPEC, 30> {
        EFUSE_CK_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_ck_en(&mut self) -> LP_CPU_CK_EN_W<CLK_EN_SPEC, 31> {
        LP_CPU_CK_EN_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0x7f80_0000"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f80_0000;
}
