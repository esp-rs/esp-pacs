#[doc = "Register `SPI_SLAVE3` reader"]
pub type R = crate::R<SPI_SLAVE3_SPEC>;
#[doc = "Register `SPI_SLAVE3` writer"]
pub type W = crate::W<SPI_SLAVE3_SPEC>;
#[doc = "Field `slv_rdbuf_cmd_value` reader - In slave mode, it is the value of \"read-buffer\" command"]
pub type SLV_RDBUF_CMD_VALUE_R = crate::FieldReader;
#[doc = "Field `slv_rdbuf_cmd_value` writer - In slave mode, it is the value of \"read-buffer\" command"]
pub type SLV_RDBUF_CMD_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `slv_wrbuf_cmd_value` reader - In slave mode, it is the value of \"write-buffer\" command"]
pub type SLV_WRBUF_CMD_VALUE_R = crate::FieldReader;
#[doc = "Field `slv_wrbuf_cmd_value` writer - In slave mode, it is the value of \"write-buffer\" command"]
pub type SLV_WRBUF_CMD_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `slv_rdsta_cmd_value` reader - In slave mode, it is the value of \"read-status\" command"]
pub type SLV_RDSTA_CMD_VALUE_R = crate::FieldReader;
#[doc = "Field `slv_rdsta_cmd_value` writer - In slave mode, it is the value of \"read-status\" command"]
pub type SLV_RDSTA_CMD_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `slv_wrsta_cmd_value` reader - In slave mode, it is the value of \"write-status\" command"]
pub type SLV_WRSTA_CMD_VALUE_R = crate::FieldReader;
#[doc = "Field `slv_wrsta_cmd_value` writer - In slave mode, it is the value of \"write-status\" command"]
pub type SLV_WRSTA_CMD_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In slave mode, it is the value of \"read-buffer\" command"]
    #[inline(always)]
    pub fn slv_rdbuf_cmd_value(&self) -> SLV_RDBUF_CMD_VALUE_R {
        SLV_RDBUF_CMD_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In slave mode, it is the value of \"write-buffer\" command"]
    #[inline(always)]
    pub fn slv_wrbuf_cmd_value(&self) -> SLV_WRBUF_CMD_VALUE_R {
        SLV_WRBUF_CMD_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In slave mode, it is the value of \"read-status\" command"]
    #[inline(always)]
    pub fn slv_rdsta_cmd_value(&self) -> SLV_RDSTA_CMD_VALUE_R {
        SLV_RDSTA_CMD_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In slave mode, it is the value of \"write-status\" command"]
    #[inline(always)]
    pub fn slv_wrsta_cmd_value(&self) -> SLV_WRSTA_CMD_VALUE_R {
        SLV_WRSTA_CMD_VALUE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE3")
            .field(
                "slv_wrsta_cmd_value",
                &format_args!("{}", self.slv_wrsta_cmd_value().bits()),
            )
            .field(
                "slv_rdsta_cmd_value",
                &format_args!("{}", self.slv_rdsta_cmd_value().bits()),
            )
            .field(
                "slv_wrbuf_cmd_value",
                &format_args!("{}", self.slv_wrbuf_cmd_value().bits()),
            )
            .field(
                "slv_rdbuf_cmd_value",
                &format_args!("{}", self.slv_rdbuf_cmd_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In slave mode, it is the value of \"read-buffer\" command"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_cmd_value(&mut self) -> SLV_RDBUF_CMD_VALUE_W<SPI_SLAVE3_SPEC, 0> {
        SLV_RDBUF_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 8:15 - In slave mode, it is the value of \"write-buffer\" command"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_cmd_value(&mut self) -> SLV_WRBUF_CMD_VALUE_W<SPI_SLAVE3_SPEC, 8> {
        SLV_WRBUF_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 16:23 - In slave mode, it is the value of \"read-status\" command"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_cmd_value(&mut self) -> SLV_RDSTA_CMD_VALUE_W<SPI_SLAVE3_SPEC, 16> {
        SLV_RDSTA_CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 24:31 - In slave mode, it is the value of \"write-status\" command"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_cmd_value(&mut self) -> SLV_WRSTA_CMD_VALUE_W<SPI_SLAVE3_SPEC, 24> {
        SLV_WRSTA_CMD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "In slave mode, it is the value of \"write-status\" command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE3_SPEC;
impl crate::RegisterSpec for SPI_SLAVE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave3::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave3::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE3 to value 0"]
impl crate::Resettable for SPI_SLAVE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
