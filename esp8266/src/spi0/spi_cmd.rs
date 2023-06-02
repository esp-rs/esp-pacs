#[doc = "Register `SPI_CMD` reader"]
pub struct R(crate::R<SPI_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CMD` writer"]
pub struct W(crate::W<SPI_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CMD_SPEC>;
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
impl From<crate::W<SPI_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_usr` reader - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
pub type SPI_USR_R = crate::BitReader;
#[doc = "Field `spi_usr` writer - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
pub type SPI_USR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_hpm` reader - "]
pub type SPI_HPM_R = crate::BitReader;
#[doc = "Field `spi_hpm` writer - "]
pub type SPI_HPM_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_res` reader - "]
pub type SPI_RES_R = crate::BitReader;
#[doc = "Field `spi_res` writer - "]
pub type SPI_RES_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_dp` reader - "]
pub type SPI_DP_R = crate::BitReader;
#[doc = "Field `spi_dp` writer - "]
pub type SPI_DP_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_ce` reader - "]
pub type SPI_CE_R = crate::BitReader;
#[doc = "Field `spi_ce` writer - "]
pub type SPI_CE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_be` reader - "]
pub type SPI_BE_R = crate::BitReader;
#[doc = "Field `spi_be` writer - "]
pub type SPI_BE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_se` reader - "]
pub type SPI_SE_R = crate::BitReader;
#[doc = "Field `spi_se` writer - "]
pub type SPI_SE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_pp` reader - "]
pub type SPI_PP_R = crate::BitReader;
#[doc = "Field `spi_pp` writer - "]
pub type SPI_PP_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_write_sr` reader - "]
pub type SPI_WRITE_SR_R = crate::BitReader;
#[doc = "Field `spi_write_sr` writer - "]
pub type SPI_WRITE_SR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_read_sr` reader - "]
pub type SPI_READ_SR_R = crate::BitReader;
#[doc = "Field `spi_read_sr` writer - "]
pub type SPI_READ_SR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_read_id` reader - "]
pub type SPI_READ_ID_R = crate::BitReader;
#[doc = "Field `spi_read_id` writer - "]
pub type SPI_READ_ID_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_write_disable` reader - "]
pub type SPI_WRITE_DISABLE_R = crate::BitReader;
#[doc = "Field `spi_write_disable` writer - "]
pub type SPI_WRITE_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_write_enable` reader - "]
pub type SPI_WRITE_ENABLE_R = crate::BitReader;
#[doc = "Field `spi_write_enable` writer - "]
pub type SPI_WRITE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
#[doc = "Field `spi_read` reader - "]
pub type SPI_READ_R = crate::BitReader;
#[doc = "Field `spi_read` writer - "]
pub type SPI_READ_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CMD_SPEC, O>;
impl R {
    #[doc = "Bit 18 - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
    #[inline(always)]
    pub fn spi_usr(&self) -> SPI_USR_R {
        SPI_USR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_hpm(&self) -> SPI_HPM_R {
        SPI_HPM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_res(&self) -> SPI_RES_R {
        SPI_RES_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_dp(&self) -> SPI_DP_R {
        SPI_DP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_ce(&self) -> SPI_CE_R {
        SPI_CE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_be(&self) -> SPI_BE_R {
        SPI_BE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_se(&self) -> SPI_SE_R {
        SPI_SE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_pp(&self) -> SPI_PP_R {
        SPI_PP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_write_sr(&self) -> SPI_WRITE_SR_R {
        SPI_WRITE_SR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_read_sr(&self) -> SPI_READ_SR_R {
        SPI_READ_SR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_read_id(&self) -> SPI_READ_ID_R {
        SPI_READ_ID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_write_disable(&self) -> SPI_WRITE_DISABLE_R {
        SPI_WRITE_DISABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_write_enable(&self) -> SPI_WRITE_ENABLE_R {
        SPI_WRITE_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_read(&self) -> SPI_READ_R {
        SPI_READ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CMD")
            .field("spi_usr", &format_args!("{}", self.spi_usr().bit()))
            .field("spi_read", &format_args!("{}", self.spi_read().bit()))
            .field(
                "spi_write_enable",
                &format_args!("{}", self.spi_write_enable().bit()),
            )
            .field(
                "spi_write_disable",
                &format_args!("{}", self.spi_write_disable().bit()),
            )
            .field("spi_read_id", &format_args!("{}", self.spi_read_id().bit()))
            .field("spi_read_sr", &format_args!("{}", self.spi_read_sr().bit()))
            .field(
                "spi_write_sr",
                &format_args!("{}", self.spi_write_sr().bit()),
            )
            .field("spi_pp", &format_args!("{}", self.spi_pp().bit()))
            .field("spi_se", &format_args!("{}", self.spi_se().bit()))
            .field("spi_be", &format_args!("{}", self.spi_be().bit()))
            .field("spi_ce", &format_args!("{}", self.spi_ce().bit()))
            .field("spi_dp", &format_args!("{}", self.spi_dp().bit()))
            .field("spi_res", &format_args!("{}", self.spi_res().bit()))
            .field("spi_hpm", &format_args!("{}", self.spi_hpm().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - In the master mode, it is the start bit of a single operation. Self-clear by hardware"]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr(&mut self) -> SPI_USR_W<18> {
        SPI_USR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn spi_hpm(&mut self) -> SPI_HPM_W<19> {
        SPI_HPM_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn spi_res(&mut self) -> SPI_RES_W<20> {
        SPI_RES_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dp(&mut self) -> SPI_DP_W<21> {
        SPI_DP_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ce(&mut self) -> SPI_CE_W<22> {
        SPI_CE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn spi_be(&mut self) -> SPI_BE_W<23> {
        SPI_BE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn spi_se(&mut self) -> SPI_SE_W<24> {
        SPI_SE_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn spi_pp(&mut self) -> SPI_PP_W<25> {
        SPI_PP_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn spi_write_sr(&mut self) -> SPI_WRITE_SR_W<26> {
        SPI_WRITE_SR_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn spi_read_sr(&mut self) -> SPI_READ_SR_W<27> {
        SPI_READ_SR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn spi_read_id(&mut self) -> SPI_READ_ID_W<28> {
        SPI_READ_ID_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn spi_write_disable(&mut self) -> SPI_WRITE_DISABLE_W<29> {
        SPI_WRITE_DISABLE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn spi_write_enable(&mut self) -> SPI_WRITE_ENABLE_W<30> {
        SPI_WRITE_ENABLE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn spi_read(&mut self) -> SPI_READ_W<31> {
        SPI_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In the master mode, it is the start bit of a single operation. Self-clear by hardware\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cmd](index.html) module"]
pub struct SPI_CMD_SPEC;
impl crate::RegisterSpec for SPI_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cmd::R](R) reader structure"]
impl crate::Readable for SPI_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cmd::W](W) writer structure"]
impl crate::Writable for SPI_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CMD to value 0"]
impl crate::Resettable for SPI_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
