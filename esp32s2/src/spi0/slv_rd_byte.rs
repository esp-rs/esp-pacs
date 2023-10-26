#[doc = "Register `SLV_RD_BYTE` reader"]
pub type R = crate::R<SLV_RD_BYTE_SPEC>;
#[doc = "Register `SLV_RD_BYTE` writer"]
pub type W = crate::W<SLV_RD_BYTE_SPEC>;
#[doc = "Field `SLV_DATA_BYTELEN` reader - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]."]
pub type SLV_DATA_BYTELEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_DATA_BYTELEN` writer - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]."]
pub type SLV_DATA_BYTELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `SLV_RDDMA_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SLV_RDDMA_BYTELEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDDMA_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SLV_RDDMA_BYTELEN_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLV_WRDMA_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SLV_WRDMA_BYTELEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRDMA_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SLV_WRDMA_BYTELEN_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLV_RDBUF_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SLV_RDBUF_BYTELEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SLV_RDBUF_BYTELEN_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLV_WRBUF_BYTELEN_EN` reader - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SLV_WRBUF_BYTELEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_BYTELEN_EN` writer - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SLV_WRBUF_BYTELEN_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` reader - The magic value of BM table in master DMA seg-trans."]
pub type DMA_SEG_MAGIC_VALUE_R = crate::FieldReader;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` writer - The magic value of BM table in master DMA seg-trans."]
pub type DMA_SEG_MAGIC_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SLV_RD_DMA_DONE` reader - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_DMA_DONE_R = crate::BitReader;
#[doc = "Field `SLV_RD_DMA_DONE` writer - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_RD_DMA_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USR_CONF` reader - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type USR_CONF_R = crate::BitReader;
#[doc = "Field `USR_CONF` writer - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type USR_CONF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:19 - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]."]
    #[inline(always)]
    pub fn slv_data_bytelen(&self) -> SLV_DATA_BYTELEN_R {
        SLV_DATA_BYTELEN_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_rddma_bytelen_en(&self) -> SLV_RDDMA_BYTELEN_EN_R {
        SLV_RDDMA_BYTELEN_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_wrdma_bytelen_en(&self) -> SLV_WRDMA_BYTELEN_EN_R {
        SLV_WRDMA_BYTELEN_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_rdbuf_bytelen_en(&self) -> SLV_RDBUF_BYTELEN_EN_R {
        SLV_RDBUF_BYTELEN_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_wrbuf_bytelen_en(&self) -> SLV_WRBUF_BYTELEN_EN_R {
        SLV_WRBUF_BYTELEN_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_rd_dma_done(&self) -> SLV_RD_DMA_DONE_R {
        SLV_RD_DMA_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_RD_BYTE")
            .field(
                "slv_data_bytelen",
                &format_args!("{}", self.slv_data_bytelen().bits()),
            )
            .field(
                "slv_rddma_bytelen_en",
                &format_args!("{}", self.slv_rddma_bytelen_en().bit()),
            )
            .field(
                "slv_wrdma_bytelen_en",
                &format_args!("{}", self.slv_wrdma_bytelen_en().bit()),
            )
            .field(
                "slv_rdbuf_bytelen_en",
                &format_args!("{}", self.slv_rdbuf_bytelen_en().bit()),
            )
            .field(
                "slv_wrbuf_bytelen_en",
                &format_args!("{}", self.slv_wrbuf_bytelen_en().bit()),
            )
            .field(
                "dma_seg_magic_value",
                &format_args!("{}", self.dma_seg_magic_value().bits()),
            )
            .field(
                "slv_rd_dma_done",
                &format_args!("{}", self.slv_rd_dma_done().bit()),
            )
            .field("usr_conf", &format_args!("{}", self.usr_conf().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLV_RD_BYTE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The full-duplex or half-duplex data byte length of the last SPI transfer in slave mode. In half-duplex mode, this value is controlled by bits \\[23:20\\]."]
    #[inline(always)]
    #[must_use]
    pub fn slv_data_bytelen(&mut self) -> SLV_DATA_BYTELEN_W<SLV_RD_BYTE_SPEC, 0> {
        SLV_DATA_BYTELEN_W::new(self)
    }
    #[doc = "Bit 20 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rddma_bytelen_en(&mut self) -> SLV_RDDMA_BYTELEN_EN_W<SLV_RD_BYTE_SPEC, 20> {
        SLV_RDDMA_BYTELEN_EN_W::new(self)
    }
    #[doc = "Bit 21 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrdma_bytelen_en(&mut self) -> SLV_WRDMA_BYTELEN_EN_W<SLV_RD_BYTE_SPEC, 21> {
        SLV_WRDMA_BYTELEN_EN_W::new(self)
    }
    #[doc = "Bit 22 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_bytelen_en(&mut self) -> SLV_RDBUF_BYTELEN_EN_W<SLV_RD_BYTE_SPEC, 22> {
        SLV_RDBUF_BYTELEN_EN_W::new(self)
    }
    #[doc = "Bit 23 - 1: SPI_SLV_DATA_BYTELEN stores data byte length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_bytelen_en(&mut self) -> SLV_WRBUF_BYTELEN_EN_W<SLV_RD_BYTE_SPEC, 23> {
        SLV_WRBUF_BYTELEN_EN_W::new(self)
    }
    #[doc = "Bits 24:27 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W<SLV_RD_BYTE_SPEC, 24> {
        DMA_SEG_MAGIC_VALUE_W::new(self)
    }
    #[doc = "Bit 30 - The interrupt raw bit for the completion of Rd-DMA operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_dma_done(&mut self) -> SLV_RD_DMA_DONE_W<SLV_RD_BYTE_SPEC, 30> {
        SLV_RD_DMA_DONE_W::new(self)
    }
    #[doc = "Bit 31 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    #[must_use]
    pub fn usr_conf(&mut self) -> USR_CONF_W<SLV_RD_BYTE_SPEC, 31> {
        USR_CONF_W::new(self)
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
#[doc = "SPI interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_rd_byte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rd_byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_RD_BYTE_SPEC;
impl crate::RegisterSpec for SLV_RD_BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_rd_byte::R`](R) reader structure"]
impl crate::Readable for SLV_RD_BYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_rd_byte::W`](W) writer structure"]
impl crate::Writable for SLV_RD_BYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLV_RD_BYTE to value 0x0a00_0000"]
impl crate::Resettable for SLV_RD_BYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00_0000;
}
