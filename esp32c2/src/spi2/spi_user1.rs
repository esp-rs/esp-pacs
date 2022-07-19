#[doc = "Register `SPI_USER1` reader"]
pub struct R(crate::R<SPI_USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_USER1` writer"]
pub struct W(crate::W<SPI_USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_USER1_SPEC>;
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
impl From<crate::W<SPI_USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type SPI_USR_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type SPI_USR_DUMMY_CYCLELEN_W<'a> = crate::FieldWriter<'a, u32, SPI_USER1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SPI_MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_WFULL_ERR_END_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_WFULL_ERR_END_EN_W<'a> = crate::BitWriter<'a, u32, SPI_USER1_SPEC, bool, 16>;
#[doc = "Field `SPI_CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type SPI_CS_SETUP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type SPI_CS_SETUP_TIME_W<'a> = crate::FieldWriter<'a, u32, SPI_USER1_SPEC, u8, u8, 5, 17>;
#[doc = "Field `SPI_CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type SPI_CS_HOLD_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type SPI_CS_HOLD_TIME_W<'a> = crate::FieldWriter<'a, u32, SPI_USER1_SPEC, u8, u8, 5, 22>;
#[doc = "Field `SPI_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_ADDR_BITLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_ADDR_BITLEN_W<'a> = crate::FieldWriter<'a, u32, SPI_USER1_SPEC, u8, u8, 5, 27>;
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy_cyclelen(&self) -> SPI_USR_DUMMY_CYCLELEN_R {
        SPI_USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn spi_mst_wfull_err_end_en(&self) -> SPI_MST_WFULL_ERR_END_EN_R {
        SPI_MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_setup_time(&self) -> SPI_CS_SETUP_TIME_R {
        SPI_CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_hold_time(&self) -> SPI_CS_HOLD_TIME_R {
        SPI_CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_addr_bitlen(&self) -> SPI_USR_ADDR_BITLEN_R {
        SPI_USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_dummy_cyclelen(&mut self) -> SPI_USR_DUMMY_CYCLELEN_W {
        SPI_USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn spi_mst_wfull_err_end_en(&mut self) -> SPI_MST_WFULL_ERR_END_EN_W {
        SPI_MST_WFULL_ERR_END_EN_W::new(self)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_setup_time(&mut self) -> SPI_CS_SETUP_TIME_W {
        SPI_CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_hold_time(&mut self) -> SPI_CS_HOLD_TIME_W {
        SPI_CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_usr_addr_bitlen(&mut self) -> SPI_USR_ADDR_BITLEN_W {
        SPI_USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI USER control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user1](index.html) module"]
pub struct SPI_USER1_SPEC;
impl crate::RegisterSpec for SPI_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_user1::R](R) reader structure"]
impl crate::Readable for SPI_USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_user1::W](W) writer structure"]
impl crate::Writable for SPI_USER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_USER1 to value 0xb841_0007"]
impl crate::Resettable for SPI_USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb841_0007
    }
}
