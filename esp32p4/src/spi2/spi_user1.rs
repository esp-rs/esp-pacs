#[doc = "Register `SPI_USER1` reader"]
pub type R = crate::R<SPI_USER1_SPEC>;
#[doc = "Register `SPI_USER1` writer"]
pub type W = crate::W<SPI_USER1_SPEC>;
#[doc = "Field `SPI_USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type SPI_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type SPI_USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_WFULL_ERR_END_EN_R = crate::BitReader;
#[doc = "Field `SPI_MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type SPI_MST_WFULL_ERR_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type SPI_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type SPI_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type SPI_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type SPI_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type SPI_USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER1")
            .field(
                "spi_usr_dummy_cyclelen",
                &format_args!("{}", self.spi_usr_dummy_cyclelen().bits()),
            )
            .field(
                "spi_mst_wfull_err_end_en",
                &format_args!("{}", self.spi_mst_wfull_err_end_en().bit()),
            )
            .field(
                "spi_cs_setup_time",
                &format_args!("{}", self.spi_cs_setup_time().bits()),
            )
            .field(
                "spi_cs_hold_time",
                &format_args!("{}", self.spi_cs_hold_time().bits()),
            )
            .field(
                "spi_usr_addr_bitlen",
                &format_args!("{}", self.spi_usr_addr_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_dummy_cyclelen(&mut self) -> SPI_USR_DUMMY_CYCLELEN_W<SPI_USER1_SPEC> {
        SPI_USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_wfull_err_end_en(&mut self) -> SPI_MST_WFULL_ERR_END_EN_W<SPI_USER1_SPEC> {
        SPI_MST_WFULL_ERR_END_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_setup_time(&mut self) -> SPI_CS_SETUP_TIME_W<SPI_USER1_SPEC> {
        SPI_CS_SETUP_TIME_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs_hold_time(&mut self) -> SPI_CS_HOLD_TIME_W<SPI_USER1_SPEC> {
        SPI_CS_HOLD_TIME_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_usr_addr_bitlen(&mut self) -> SPI_USR_ADDR_BITLEN_W<SPI_USER1_SPEC> {
        SPI_USR_ADDR_BITLEN_W::new(self, 27)
    }
}
#[doc = "SPI USER control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER1_SPEC;
impl crate::RegisterSpec for SPI_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user1::R`](R) reader structure"]
impl crate::Readable for SPI_USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user1::W`](W) writer structure"]
impl crate::Writable for SPI_USER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_USER1 to value 0xb841_0007"]
impl crate::Resettable for SPI_USER1_SPEC {
    const RESET_VALUE: u32 = 0xb841_0007;
}
