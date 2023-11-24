#[doc = "Register `SPI_SLAVE1` reader"]
pub type R = crate::R<SPI_SLAVE1_SPEC>;
#[doc = "Register `SPI_SLAVE1` writer"]
pub type W = crate::W<SPI_SLAVE1_SPEC>;
#[doc = "Field `slv_rdbuf_dummy_en` reader - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-buffer\" operations."]
pub type SLV_RDBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `slv_rdbuf_dummy_en` writer - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-buffer\" operations."]
pub type SLV_RDBUF_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_wrbuf_dummy_en` reader - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-buffer\" operations."]
pub type SLV_WRBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `slv_wrbuf_dummy_en` writer - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-buffer\" operations."]
pub type SLV_WRBUF_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_rdsta_dummy_en` reader - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-status\" operations."]
pub type SLV_RDSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `slv_rdsta_dummy_en` writer - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-status\" operations."]
pub type SLV_RDSTA_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_wrsta_dummy_en` reader - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-status\" operations."]
pub type SLV_WRSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `slv_wrsta_dummy_en` writer - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-status\" operations."]
pub type SLV_WRSTA_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_wr_addr_bitlen` reader - In the slave mode, it is the address length in bits for \"write-buffer\" operation. The register value shall be(bit_num-1)"]
pub type SLV_WR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `slv_wr_addr_bitlen` writer - In the slave mode, it is the address length in bits for \"write-buffer\" operation. The register value shall be(bit_num-1)"]
pub type SLV_WR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `slv_rd_addr_bitlen` reader - In the slave mode, it is the address length in bits for \"read-buffer\" operation. The register value shall be(bit_num-1)"]
pub type SLV_RD_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `slv_rd_addr_bitlen` writer - In the slave mode, it is the address length in bits for \"read-buffer\" operation. The register value shall be(bit_num-1)"]
pub type SLV_RD_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `slv_buf_bitlen` reader - In the slave mode, it is the length in bits for \"write-buffer\" and \"read-buffer\" operations. The register value shallbe (bit_num-1)"]
pub type SLV_BUF_BITLEN_R = crate::FieldReader<u16>;
#[doc = "Field `slv_buf_bitlen` writer - In the slave mode, it is the length in bits for \"write-buffer\" and \"read-buffer\" operations. The register value shallbe (bit_num-1)"]
pub type SLV_BUF_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `spi_status_read` reader - Enable spi slave status"]
pub type SPI_STATUS_READ_R = crate::BitReader;
#[doc = "Field `spi_status_read` writer - Enable spi slave status"]
pub type SPI_STATUS_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi_status_fast_enable` reader - Enable fast spi slave status"]
pub type SPI_STATUS_FAST_ENABLE_R = crate::BitReader;
#[doc = "Field `spi_status_fast_enable` writer - Enable fast spi slave status"]
pub type SPI_STATUS_FAST_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_status_bitlen` reader - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
pub type SLV_STATUS_BITLEN_R = crate::FieldReader;
#[doc = "Field `slv_status_bitlen` writer - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
pub type SLV_STATUS_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-buffer\" operations."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_en(&self) -> SLV_RDBUF_DUMMY_EN_R {
        SLV_RDBUF_DUMMY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-buffer\" operations."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_en(&self) -> SLV_WRBUF_DUMMY_EN_R {
        SLV_WRBUF_DUMMY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-status\" operations."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_en(&self) -> SLV_RDSTA_DUMMY_EN_R {
        SLV_RDSTA_DUMMY_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-status\" operations."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_en(&self) -> SLV_WRSTA_DUMMY_EN_R {
        SLV_WRSTA_DUMMY_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - In the slave mode, it is the address length in bits for \"write-buffer\" operation. The register value shall be(bit_num-1)"]
    #[inline(always)]
    pub fn slv_wr_addr_bitlen(&self) -> SLV_WR_ADDR_BITLEN_R {
        SLV_WR_ADDR_BITLEN_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - In the slave mode, it is the address length in bits for \"read-buffer\" operation. The register value shall be(bit_num-1)"]
    #[inline(always)]
    pub fn slv_rd_addr_bitlen(&self) -> SLV_RD_ADDR_BITLEN_R {
        SLV_RD_ADDR_BITLEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:24 - In the slave mode, it is the length in bits for \"write-buffer\" and \"read-buffer\" operations. The register value shallbe (bit_num-1)"]
    #[inline(always)]
    pub fn slv_buf_bitlen(&self) -> SLV_BUF_BITLEN_R {
        SLV_BUF_BITLEN_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Enable spi slave status"]
    #[inline(always)]
    pub fn spi_status_read(&self) -> SPI_STATUS_READ_R {
        SPI_STATUS_READ_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable fast spi slave status"]
    #[inline(always)]
    pub fn spi_status_fast_enable(&self) -> SPI_STATUS_FAST_ENABLE_R {
        SPI_STATUS_FAST_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
    #[inline(always)]
    pub fn slv_status_bitlen(&self) -> SLV_STATUS_BITLEN_R {
        SLV_STATUS_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE1")
            .field(
                "slv_status_bitlen",
                &format_args!("{}", self.slv_status_bitlen().bits()),
            )
            .field(
                "slv_buf_bitlen",
                &format_args!("{}", self.slv_buf_bitlen().bits()),
            )
            .field(
                "slv_rd_addr_bitlen",
                &format_args!("{}", self.slv_rd_addr_bitlen().bits()),
            )
            .field(
                "slv_wr_addr_bitlen",
                &format_args!("{}", self.slv_wr_addr_bitlen().bits()),
            )
            .field(
                "slv_wrsta_dummy_en",
                &format_args!("{}", self.slv_wrsta_dummy_en().bit()),
            )
            .field(
                "slv_rdsta_dummy_en",
                &format_args!("{}", self.slv_rdsta_dummy_en().bit()),
            )
            .field(
                "slv_wrbuf_dummy_en",
                &format_args!("{}", self.slv_wrbuf_dummy_en().bit()),
            )
            .field(
                "slv_rdbuf_dummy_en",
                &format_args!("{}", self.slv_rdbuf_dummy_en().bit()),
            )
            .field(
                "spi_status_fast_enable",
                &format_args!("{}", self.spi_status_fast_enable().bit()),
            )
            .field(
                "spi_status_read",
                &format_args!("{}", self.spi_status_read().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-buffer\" operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_en(&mut self) -> SLV_RDBUF_DUMMY_EN_W<SPI_SLAVE1_SPEC> {
        SLV_RDBUF_DUMMY_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-buffer\" operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_en(&mut self) -> SLV_WRBUF_DUMMY_EN_W<SPI_SLAVE1_SPEC> {
        SLV_WRBUF_DUMMY_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - In the slave mode, it is the enable bit of \"dummy\" phase for \"read-status\" operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_en(&mut self) -> SLV_RDSTA_DUMMY_EN_W<SPI_SLAVE1_SPEC> {
        SLV_RDSTA_DUMMY_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - In the slave mode, it is the enable bit of \"dummy\" phase for \"write-status\" operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_en(&mut self) -> SLV_WRSTA_DUMMY_EN_W<SPI_SLAVE1_SPEC> {
        SLV_WRSTA_DUMMY_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:9 - In the slave mode, it is the address length in bits for \"write-buffer\" operation. The register value shall be(bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_addr_bitlen(&mut self) -> SLV_WR_ADDR_BITLEN_W<SPI_SLAVE1_SPEC> {
        SLV_WR_ADDR_BITLEN_W::new(self, 4)
    }
    #[doc = "Bits 10:15 - In the slave mode, it is the address length in bits for \"read-buffer\" operation. The register value shall be(bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_addr_bitlen(&mut self) -> SLV_RD_ADDR_BITLEN_W<SPI_SLAVE1_SPEC> {
        SLV_RD_ADDR_BITLEN_W::new(self, 10)
    }
    #[doc = "Bits 16:24 - In the slave mode, it is the length in bits for \"write-buffer\" and \"read-buffer\" operations. The register value shallbe (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_buf_bitlen(&mut self) -> SLV_BUF_BITLEN_W<SPI_SLAVE1_SPEC> {
        SLV_BUF_BITLEN_W::new(self, 16)
    }
    #[doc = "Bit 25 - Enable spi slave status"]
    #[inline(always)]
    #[must_use]
    pub fn spi_status_read(&mut self) -> SPI_STATUS_READ_W<SPI_SLAVE1_SPEC> {
        SPI_STATUS_READ_W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable fast spi slave status"]
    #[inline(always)]
    #[must_use]
    pub fn spi_status_fast_enable(&mut self) -> SPI_STATUS_FAST_ENABLE_W<SPI_SLAVE1_SPEC> {
        SPI_STATUS_FAST_ENABLE_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_bitlen(&mut self) -> SLV_STATUS_BITLEN_W<SPI_SLAVE1_SPEC> {
        SLV_STATUS_BITLEN_W::new(self, 27)
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
#[doc = "In the slave mode, it is the length in bits for \"write-status\" and \"read-status\" operations. The register valueshall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE1_SPEC;
impl crate::RegisterSpec for SPI_SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave1::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave1::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE1 to value 0"]
impl crate::Resettable for SPI_SLAVE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
