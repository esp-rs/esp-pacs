#[doc = "Register `SPI_SLAVE` reader"]
pub struct R(crate::R<SPI_SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SLAVE` writer"]
pub struct W(crate::W<SPI_SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SLAVE_SPEC>;
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
impl From<crate::W<SPI_SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type SPI_CLK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type SPI_CLK_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_SLAVE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SPI_CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type SPI_CLK_MODE_13_R = crate::BitReader<bool>;
#[doc = "Field `SPI_CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type SPI_CLK_MODE_13_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 2>;
#[doc = "Field `SPI_RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type SPI_RSCK_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SPI_RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type SPI_RSCK_DATA_OUT_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 3>;
#[doc = "Field `SPI_SLV_RDDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SPI_SLV_RDDMA_BITLEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_RDDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
pub type SPI_SLV_RDDMA_BITLEN_EN_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 8>;
#[doc = "Field `SPI_SLV_WRDMA_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SPI_SLV_WRDMA_BITLEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_WRDMA_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
pub type SPI_SLV_WRDMA_BITLEN_EN_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 9>;
#[doc = "Field `SPI_SLV_RDBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SPI_SLV_RDBUF_BITLEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_RDBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type SPI_SLV_RDBUF_BITLEN_EN_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 10>;
#[doc = "Field `SPI_SLV_WRBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SPI_SLV_WRBUF_BITLEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_WRBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type SPI_SLV_WRBUF_BITLEN_EN_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 11>;
#[doc = "Field `SPI_DMA_SEG_MAGIC_VALUE` reader - The magic value of BM table in master DMA seg-trans."]
pub type SPI_DMA_SEG_MAGIC_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_DMA_SEG_MAGIC_VALUE` writer - The magic value of BM table in master DMA seg-trans."]
pub type SPI_DMA_SEG_MAGIC_VALUE_W<'a> = crate::FieldWriter<'a, u32, SPI_SLAVE_SPEC, u8, u8, 4, 22>;
#[doc = "Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 26>;
#[doc = "Field `SPI_SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type SPI_SOFT_RESET_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 27>;
#[doc = "Field `SPI_USR_CONF` reader - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type SPI_USR_CONF_R = crate::BitReader<bool>;
#[doc = "Field `SPI_USR_CONF` writer - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
pub type SPI_USR_CONF_W<'a> = crate::BitWriter<'a, u32, SPI_SLAVE_SPEC, bool, 28>;
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
    #[doc = "Bits 22:25 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn spi_dma_seg_magic_value(&self) -> SPI_DMA_SEG_MAGIC_VALUE_R {
        SPI_DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn spi_usr_conf(&self) -> SPI_USR_CONF_R {
        SPI_USR_CONF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_clk_mode(&mut self) -> SPI_CLK_MODE_W {
        SPI_CLK_MODE_W::new(self)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn spi_clk_mode_13(&mut self) -> SPI_CLK_MODE_13_W {
        SPI_CLK_MODE_13_W::new(self)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn spi_rsck_data_out(&mut self) -> SPI_RSCK_DATA_OUT_W {
        SPI_RSCK_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 8 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    #[inline(always)]
    pub fn spi_slv_rddma_bitlen_en(&mut self) -> SPI_SLV_RDDMA_BITLEN_EN_W {
        SPI_SLV_RDDMA_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 9 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    #[inline(always)]
    pub fn spi_slv_wrdma_bitlen_en(&mut self) -> SPI_SLV_WRDMA_BITLEN_EN_W {
        SPI_SLV_WRDMA_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn spi_slv_rdbuf_bitlen_en(&mut self) -> SPI_SLV_RDBUF_BITLEN_EN_W {
        SPI_SLV_RDBUF_BITLEN_EN_W::new(self)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn spi_slv_wrbuf_bitlen_en(&mut self) -> SPI_SLV_WRBUF_BITLEN_EN_W {
        SPI_SLV_WRBUF_BITLEN_EN_W::new(self)
    }
    #[doc = "Bits 22:25 - The magic value of BM table in master DMA seg-trans."]
    #[inline(always)]
    pub fn spi_dma_seg_magic_value(&mut self) -> SPI_DMA_SEG_MAGIC_VALUE_W {
        SPI_DMA_SEG_MAGIC_VALUE_W::new(self)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 27 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_soft_reset(&mut self) -> SPI_SOFT_RESET_W {
        SPI_SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 28 - 1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    #[inline(always)]
    pub fn spi_usr_conf(&mut self) -> SPI_USR_CONF_W {
        SPI_USR_CONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_slave](index.html) module"]
pub struct SPI_SLAVE_SPEC;
impl crate::RegisterSpec for SPI_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_slave::R](R) reader structure"]
impl crate::Readable for SPI_SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_slave::W](W) writer structure"]
impl crate::Writable for SPI_SLAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_SLAVE to value 0x0280_0000"]
impl crate::Resettable for SPI_SLAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0280_0000
    }
}
