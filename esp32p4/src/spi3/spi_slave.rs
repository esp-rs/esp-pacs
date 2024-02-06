#[doc = "Register `SPI_SLAVE` reader"]
pub type R = crate::R<SPI_SLAVE_SPEC>;
#[doc = "Register `SPI_SLAVE` writer"]
pub type W = crate::W<SPI_SLAVE_SPEC>;
#[doc = "Field `SPI_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type SPI_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type SPI_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type SPI_CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `SPI_CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type SPI_CLK_MODE_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type SPI_RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `SPI_RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type SPI_RSCK_DATA_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RDDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SPI_SLV_RDDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RDDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SPI_SLV_RDDMA_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WRDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SPI_SLV_WRDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WRDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SPI_SLV_WRDMA_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RDBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SPI_SLV_RDBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RDBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SPI_SLV_RDBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WRBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SPI_SLV_WRBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WRBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SPI_SLV_WRBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_LAST_BYTE_STRB` reader - Represents the effective bit of the last received data byte in SPI slave FD and HD mode."]
pub type SPI_SLV_LAST_BYTE_STRB_R = crate::FieldReader;
#[doc = "Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type SPI_SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_FD_WAIT_DMA_TX_DATA` reader - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
pub type SPI_MST_FD_WAIT_DMA_TX_DATA_R = crate::BitReader;
#[doc = "Field `SPI_MST_FD_WAIT_DMA_TX_DATA` writer - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
pub type SPI_MST_FD_WAIT_DMA_TX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_mode(&self) -> SPI_CLK_MODE_R {
        SPI_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn spi_clk_mode_13(&self) -> SPI_CLK_MODE_13_R {
        SPI_CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn spi_rsck_data_out(&self) -> SPI_RSCK_DATA_OUT_R {
        SPI_RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn spi_slv_rddma_bitlen_en(&self) -> SPI_SLV_RDDMA_BITLEN_EN_R {
        SPI_SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn spi_slv_wrdma_bitlen_en(&self) -> SPI_SLV_WRDMA_BITLEN_EN_R {
        SPI_SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn spi_slv_rdbuf_bitlen_en(&self) -> SPI_SLV_RDBUF_BITLEN_EN_R {
        SPI_SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn spi_slv_wrbuf_bitlen_en(&self) -> SPI_SLV_WRBUF_BITLEN_EN_R {
        SPI_SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - Represents the effective bit of the last received data byte in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn spi_slv_last_byte_strb(&self) -> SPI_SLV_LAST_BYTE_STRB_R {
        SPI_SLV_LAST_BYTE_STRB_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
    #[inline(always)]
    pub fn spi_mst_fd_wait_dma_tx_data(&self) -> SPI_MST_FD_WAIT_DMA_TX_DATA_R {
        SPI_MST_FD_WAIT_DMA_TX_DATA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE")
            .field(
                "spi_clk_mode",
                &format_args!("{}", self.spi_clk_mode().bits()),
            )
            .field(
                "spi_clk_mode_13",
                &format_args!("{}", self.spi_clk_mode_13().bit()),
            )
            .field(
                "spi_rsck_data_out",
                &format_args!("{}", self.spi_rsck_data_out().bit()),
            )
            .field(
                "spi_slv_rddma_bitlen_en",
                &format_args!("{}", self.spi_slv_rddma_bitlen_en().bit()),
            )
            .field(
                "spi_slv_wrdma_bitlen_en",
                &format_args!("{}", self.spi_slv_wrdma_bitlen_en().bit()),
            )
            .field(
                "spi_slv_rdbuf_bitlen_en",
                &format_args!("{}", self.spi_slv_rdbuf_bitlen_en().bit()),
            )
            .field(
                "spi_slv_wrbuf_bitlen_en",
                &format_args!("{}", self.spi_slv_wrbuf_bitlen_en().bit()),
            )
            .field(
                "spi_slv_last_byte_strb",
                &format_args!("{}", self.spi_slv_last_byte_strb().bits()),
            )
            .field("mode", &format_args!("{}", self.mode().bit()))
            .field(
                "spi_mst_fd_wait_dma_tx_data",
                &format_args!("{}", self.spi_mst_fd_wait_dma_tx_data().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_mode(&mut self) -> SPI_CLK_MODE_W<SPI_SLAVE_SPEC> {
        SPI_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clk_mode_13(&mut self) -> SPI_CLK_MODE_13_W<SPI_SLAVE_SPEC> {
        SPI_CLK_MODE_13_W::new(self, 2)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rsck_data_out(&mut self) -> SPI_RSCK_DATA_OUT_W<SPI_SLAVE_SPEC> {
        SPI_RSCK_DATA_OUT_W::new(self, 3)
    }
    #[doc = "Bit 8 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rddma_bitlen_en(&mut self) -> SPI_SLV_RDDMA_BITLEN_EN_W<SPI_SLAVE_SPEC> {
        SPI_SLV_RDDMA_BITLEN_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wrdma_bitlen_en(&mut self) -> SPI_SLV_WRDMA_BITLEN_EN_W<SPI_SLAVE_SPEC> {
        SPI_SLV_WRDMA_BITLEN_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rdbuf_bitlen_en(&mut self) -> SPI_SLV_RDBUF_BITLEN_EN_W<SPI_SLAVE_SPEC> {
        SPI_SLV_RDBUF_BITLEN_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wrbuf_bitlen_en(&mut self) -> SPI_SLV_WRBUF_BITLEN_EN_W<SPI_SLAVE_SPEC> {
        SPI_SLV_WRBUF_BITLEN_EN_W::new(self, 11)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SPI_SLAVE_SPEC> {
        MODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_soft_reset(&mut self) -> SPI_SOFT_RESET_W<SPI_SLAVE_SPEC> {
        SPI_SOFT_RESET_W::new(self, 27)
    }
    #[doc = "Bit 29 - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_fd_wait_dma_tx_data(&mut self) -> SPI_MST_FD_WAIT_DMA_TX_DATA_W<SPI_SLAVE_SPEC> {
        SPI_MST_FD_WAIT_DMA_TX_DATA_W::new(self, 29)
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
#[doc = "SPI slave control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE_SPEC;
impl crate::RegisterSpec for SPI_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE to value 0"]
impl crate::Resettable for SPI_SLAVE_SPEC {
    const RESET_VALUE: u32 = 0;
}
