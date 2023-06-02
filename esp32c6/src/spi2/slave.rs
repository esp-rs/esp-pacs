#[doc = "Register `SLAVE` reader"]
pub struct R(crate::R<SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE` writer"]
pub struct W(crate::W<SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPEC>;
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
impl From<crate::W<SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE_SPEC, 2, O>;
#[doc = "Field `CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SLV_RDDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SLV_RDDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SLV_RDDMA_BITLEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SLV_WRDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SLV_WRDMA_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SLV_WRDMA_BITLEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SLV_RDBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SLV_RDBUF_BITLEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SLV_WRBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SLV_WRBUF_BITLEN_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` reader - The magic value of BM table in master DMA seg-trans."]
pub type DMA_SEG_MAGIC_VALUE_R = crate::FieldReader;
#[doc = "Field `DMA_SEG_MAGIC_VALUE` writer - The magic value of BM table in master DMA seg-trans."]
pub type DMA_SEG_MAGIC_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE_SPEC, 4, O>;
#[doc = "Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `USR_CONF` reader - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type USR_CONF_R = crate::BitReader;
#[doc = "Field `USR_CONF` writer - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type USR_CONF_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `MST_FD_WAIT_DMA_TX_DATA` reader - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
pub type MST_FD_WAIT_DMA_TX_DATA_R = crate::BitReader;
#[doc = "Field `MST_FD_WAIT_DMA_TX_DATA` writer - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
pub type MST_FD_WAIT_DMA_TX_DATA_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&self) -> SLV_RDDMA_BITLEN_EN_R {
        SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&self) -> SLV_WRDMA_BITLEN_EN_R {
        SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&self) -> SLV_RDBUF_BITLEN_EN_R {
        SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&self) -> SLV_WRBUF_BITLEN_EN_R {
        SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 22:25 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
    #[inline(always)]
    pub fn mst_fd_wait_dma_tx_data(&self) -> MST_FD_WAIT_DMA_TX_DATA_R {
        MST_FD_WAIT_DMA_TX_DATA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("clk_mode", &format_args!("{}", self.clk_mode().bits()))
            .field("clk_mode_13", &format_args!("{}", self.clk_mode_13().bit()))
            .field(
                "rsck_data_out",
                &format_args!("{}", self.rsck_data_out().bit()),
            )
            .field(
                "slv_rddma_bitlen_en",
                &format_args!("{}", self.slv_rddma_bitlen_en().bit()),
            )
            .field(
                "slv_wrdma_bitlen_en",
                &format_args!("{}", self.slv_wrdma_bitlen_en().bit()),
            )
            .field(
                "slv_rdbuf_bitlen_en",
                &format_args!("{}", self.slv_rdbuf_bitlen_en().bit()),
            )
            .field(
                "slv_wrbuf_bitlen_en",
                &format_args!("{}", self.slv_wrbuf_bitlen_en().bit()),
            )
            .field(
                "dma_seg_magic_value",
                &format_args!("{}", self.dma_seg_magic_value().bits()),
            )
            .field("mode", &format_args!("{}", self.mode().bit()))
            .field("usr_conf", &format_args!("{}", self.usr_conf().bit()))
            .field(
                "mst_fd_wait_dma_tx_data",
                &format_args!("{}", self.mst_fd_wait_dma_tx_data().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<0> {
        CLK_MODE_W::new(self)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    #[must_use]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W<2> {
        CLK_MODE_13_W::new(self)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    #[must_use]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W<3> {
        RSCK_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 8 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rddma_bitlen_en(&mut self) -> SLV_RDDMA_BITLEN_EN_W<8> {
        SLV_RDDMA_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 9 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrdma_bitlen_en(&mut self) -> SLV_WRDMA_BITLEN_EN_W<9> {
        SLV_WRDMA_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_bitlen_en(&mut self) -> SLV_RDBUF_BITLEN_EN_W<10> {
        SLV_RDBUF_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_bitlen_en(&mut self) -> SLV_WRBUF_BITLEN_EN_W<11> {
        SLV_WRBUF_BITLEN_EN_W::new(self)
    }
    #[doc = "Bits 22:25 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W<22> {
        DMA_SEG_MAGIC_VALUE_W::new(self)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<26> {
        MODE_W::new(self)
    }
    #[doc = "Bit 27 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<27> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 28 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    #[must_use]
    pub fn usr_conf(&mut self) -> USR_CONF_W<28> {
        USR_CONF_W::new(self)
    }
    #[doc = "Bit 29 - In master full-duplex mode, 1: GP-SPI will wait DMA TX data is ready before starting SPI transfer. 0: GP-SPI does not wait DMA TX data before starting SPI transfer."]
    #[inline(always)]
    #[must_use]
    pub fn mst_fd_wait_dma_tx_data(&mut self) -> MST_FD_WAIT_DMA_TX_DATA_W<29> {
        MST_FD_WAIT_DMA_TX_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave](index.html) module"]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave::R](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave::W](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE to value 0x0280_0000"]
impl crate::Resettable for SLAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0280_0000;
}
