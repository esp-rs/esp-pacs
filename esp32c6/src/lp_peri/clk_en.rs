#[doc = "Register `CLK_EN` reader"]
pub struct R(crate::R<CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_EN` writer"]
pub struct W(crate::W<CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNG_CK_EN` reader - need_des"]
pub type RNG_CK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CK_EN` writer - need_des"]
pub type RNG_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `OTP_DBG_CK_EN` reader - need_des"]
pub type OTP_DBG_CK_EN_R = crate::BitReader;
#[doc = "Field `OTP_DBG_CK_EN` writer - need_des"]
pub type OTP_DBG_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `LP_UART_CK_EN` reader - need_des"]
pub type LP_UART_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_CK_EN` writer - need_des"]
pub type LP_UART_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `LP_IO_CK_EN` reader - need_des"]
pub type LP_IO_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_IO_CK_EN` writer - need_des"]
pub type LP_IO_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `LP_EXT_I2C_CK_EN` reader - need_des"]
pub type LP_EXT_I2C_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_EXT_I2C_CK_EN` writer - need_des"]
pub type LP_EXT_I2C_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `LP_ANA_I2C_CK_EN` reader - need_des"]
pub type LP_ANA_I2C_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_I2C_CK_EN` writer - need_des"]
pub type LP_ANA_I2C_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `EFUSE_CK_EN` reader - need_des"]
pub type EFUSE_CK_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_CK_EN` writer - need_des"]
pub type EFUSE_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
#[doc = "Field `LP_CPU_CK_EN` reader - need_des"]
pub type LP_CPU_CK_EN_R = crate::BitReader;
#[doc = "Field `LP_CPU_CK_EN` writer - need_des"]
pub type LP_CPU_CK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_EN_SPEC, O>;
impl R {
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
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rng_ck_en(&mut self) -> RNG_CK_EN_W<24> {
        RNG_CK_EN_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn otp_dbg_ck_en(&mut self) -> OTP_DBG_CK_EN_W<25> {
        OTP_DBG_CK_EN_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart_ck_en(&mut self) -> LP_UART_CK_EN_W<26> {
        LP_UART_CK_EN_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_io_ck_en(&mut self) -> LP_IO_CK_EN_W<27> {
        LP_IO_CK_EN_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ext_i2c_ck_en(&mut self) -> LP_EXT_I2C_CK_EN_W<28> {
        LP_EXT_I2C_CK_EN_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_i2c_ck_en(&mut self) -> LP_ANA_I2C_CK_EN_W<29> {
        LP_ANA_I2C_CK_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_ck_en(&mut self) -> EFUSE_CK_EN_W<30> {
        EFUSE_CK_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_ck_en(&mut self) -> LP_CPU_CK_EN_W<31> {
        LP_CPU_CK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_en](index.html) module"]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_en::R](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_en::W](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0x7f00_0000"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f00_0000;
}
