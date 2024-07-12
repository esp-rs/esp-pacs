#[doc = "Register `SLAVE1` reader"]
pub type R = crate::R<SLAVE1_SPEC>;
#[doc = "Register `SLAVE1` writer"]
pub type W = crate::W<SLAVE1_SPEC>;
#[doc = "Field `SLV_RDBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub type SLV_RDBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub type SLV_RDBUF_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WRBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub type SLV_WRBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub type SLV_WRBUF_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RDSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub type SLV_RDSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub type SLV_RDSTA_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WRSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub type SLV_WRSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub type SLV_WRSTA_DUMMY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_WR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_WR_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_WR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SLV_RD_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_RD_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_RD_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_RD_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SLV_STATUS_READBACK` reader - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub type SLV_STATUS_READBACK_R = crate::BitReader;
#[doc = "Field `SLV_STATUS_READBACK` writer - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub type SLV_STATUS_READBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_STATUS_FAST_EN` reader - In the slave mode enable fast read status."]
pub type SLV_STATUS_FAST_EN_R = crate::BitReader;
#[doc = "Field `SLV_STATUS_FAST_EN` writer - In the slave mode enable fast read status."]
pub type SLV_STATUS_FAST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_STATUS_BITLEN` reader - In the slave mode it is the length of status bit."]
pub type SLV_STATUS_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_STATUS_BITLEN` writer - In the slave mode it is the length of status bit."]
pub type SLV_STATUS_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_en(&self) -> SLV_RDBUF_DUMMY_EN_R {
        SLV_RDBUF_DUMMY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_en(&self) -> SLV_WRBUF_DUMMY_EN_R {
        SLV_WRBUF_DUMMY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_en(&self) -> SLV_RDSTA_DUMMY_EN_R {
        SLV_RDSTA_DUMMY_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_en(&self) -> SLV_WRSTA_DUMMY_EN_R {
        SLV_WRSTA_DUMMY_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wr_addr_bitlen(&self) -> SLV_WR_ADDR_BITLEN_R {
        SLV_WR_ADDR_BITLEN_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rd_addr_bitlen(&self) -> SLV_RD_ADDR_BITLEN_R {
        SLV_RD_ADDR_BITLEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn slv_status_readback(&self) -> SLV_STATUS_READBACK_R {
        SLV_STATUS_READBACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn slv_status_fast_en(&self) -> SLV_STATUS_FAST_EN_R {
        SLV_STATUS_FAST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn slv_status_bitlen(&self) -> SLV_STATUS_BITLEN_R {
        SLV_STATUS_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE1")
            .field("slv_rdbuf_dummy_en", &self.slv_rdbuf_dummy_en())
            .field("slv_wrbuf_dummy_en", &self.slv_wrbuf_dummy_en())
            .field("slv_rdsta_dummy_en", &self.slv_rdsta_dummy_en())
            .field("slv_wrsta_dummy_en", &self.slv_wrsta_dummy_en())
            .field("slv_wr_addr_bitlen", &self.slv_wr_addr_bitlen())
            .field("slv_rd_addr_bitlen", &self.slv_rd_addr_bitlen())
            .field("slv_status_readback", &self.slv_status_readback())
            .field("slv_status_fast_en", &self.slv_status_fast_en())
            .field("slv_status_bitlen", &self.slv_status_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_en(&mut self) -> SLV_RDBUF_DUMMY_EN_W<SLAVE1_SPEC> {
        SLV_RDBUF_DUMMY_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_en(&mut self) -> SLV_WRBUF_DUMMY_EN_W<SLAVE1_SPEC> {
        SLV_WRBUF_DUMMY_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_en(&mut self) -> SLV_RDSTA_DUMMY_EN_W<SLAVE1_SPEC> {
        SLV_RDSTA_DUMMY_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_en(&mut self) -> SLV_WRSTA_DUMMY_EN_W<SLAVE1_SPEC> {
        SLV_WRSTA_DUMMY_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_addr_bitlen(&mut self) -> SLV_WR_ADDR_BITLEN_W<SLAVE1_SPEC> {
        SLV_WR_ADDR_BITLEN_W::new(self, 4)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_addr_bitlen(&mut self) -> SLV_RD_ADDR_BITLEN_W<SLAVE1_SPEC> {
        SLV_RD_ADDR_BITLEN_W::new(self, 10)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_readback(&mut self) -> SLV_STATUS_READBACK_W<SLAVE1_SPEC> {
        SLV_STATUS_READBACK_W::new(self, 25)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_fast_en(&mut self) -> SLV_STATUS_FAST_EN_W<SLAVE1_SPEC> {
        SLV_STATUS_FAST_EN_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_bitlen(&mut self) -> SLV_STATUS_BITLEN_W<SLAVE1_SPEC> {
        SLV_STATUS_BITLEN_W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave1::R`](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave1::W`](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE1 to value 0x0200_0000"]
impl crate::Resettable for SLAVE1_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
