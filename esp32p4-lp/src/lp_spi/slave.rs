#[doc = "Register `SLAVE` reader"]
pub type R = crate::R<SLAVE_SPEC>;
#[doc = "Register `SLAVE` writer"]
pub type W = crate::W<SLAVE_SPEC>;
#[doc = "Field `LP_REG_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on. Can be configured in CONF state."]
pub type LP_REG_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `LP_REG_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on. Can be configured in CONF state."]
pub type LP_REG_CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_REG_CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type LP_REG_CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `LP_REG_CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type LP_REG_CLK_MODE_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type LP_REG_RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `LP_REG_RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type LP_REG_RSCK_DATA_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_RDBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type LP_REG_SLV_RDBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_RDBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
pub type LP_REG_SLV_RDBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_WRBUF_BITLEN_EN` reader - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type LP_REG_SLV_WRBUF_BITLEN_EN_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_WRBUF_BITLEN_EN` writer - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
pub type LP_REG_SLV_WRBUF_BITLEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLAVE_MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub type LP_REG_SLAVE_MODE_R = crate::BitReader;
#[doc = "Field `LP_REG_SLAVE_MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub type LP_REG_SLAVE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type LP_REG_SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_clk_mode(&self) -> LP_REG_CLK_MODE_R {
        LP_REG_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn lp_reg_clk_mode_13(&self) -> LP_REG_CLK_MODE_13_R {
        LP_REG_CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn lp_reg_rsck_data_out(&self) -> LP_REG_RSCK_DATA_OUT_R {
        LP_REG_RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn lp_reg_slv_rdbuf_bitlen_en(&self) -> LP_REG_SLV_RDBUF_BITLEN_EN_R {
        LP_REG_SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn lp_reg_slv_wrbuf_bitlen_en(&self) -> LP_REG_SLV_WRBUF_BITLEN_EN_R {
        LP_REG_SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn lp_reg_slave_mode(&self) -> LP_REG_SLAVE_MODE_R {
        LP_REG_SLAVE_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("lp_reg_clk_mode", &self.lp_reg_clk_mode())
            .field("lp_reg_clk_mode_13", &self.lp_reg_clk_mode_13())
            .field("lp_reg_rsck_data_out", &self.lp_reg_rsck_data_out())
            .field(
                "lp_reg_slv_rdbuf_bitlen_en",
                &self.lp_reg_slv_rdbuf_bitlen_en(),
            )
            .field(
                "lp_reg_slv_wrbuf_bitlen_en",
                &self.lp_reg_slv_wrbuf_bitlen_en(),
            )
            .field("lp_reg_slave_mode", &self.lp_reg_slave_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_clk_mode(&mut self) -> LP_REG_CLK_MODE_W<'_, SLAVE_SPEC> {
        LP_REG_CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn lp_reg_clk_mode_13(&mut self) -> LP_REG_CLK_MODE_13_W<'_, SLAVE_SPEC> {
        LP_REG_CLK_MODE_13_W::new(self, 2)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn lp_reg_rsck_data_out(&mut self) -> LP_REG_RSCK_DATA_OUT_W<'_, SLAVE_SPEC> {
        LP_REG_RSCK_DATA_OUT_W::new(self, 3)
    }
    #[doc = "Bit 10 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    #[inline(always)]
    pub fn lp_reg_slv_rdbuf_bitlen_en(&mut self) -> LP_REG_SLV_RDBUF_BITLEN_EN_W<'_, SLAVE_SPEC> {
        LP_REG_SLV_RDBUF_BITLEN_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    #[inline(always)]
    pub fn lp_reg_slv_wrbuf_bitlen_en(&mut self) -> LP_REG_SLV_WRBUF_BITLEN_EN_W<'_, SLAVE_SPEC> {
        LP_REG_SLV_WRBUF_BITLEN_EN_W::new(self, 11)
    }
    #[doc = "Bit 26 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn lp_reg_slave_mode(&mut self) -> LP_REG_SLAVE_MODE_W<'_, SLAVE_SPEC> {
        LP_REG_SLAVE_MODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_soft_reset(&mut self) -> LP_REG_SOFT_RESET_W<'_, SLAVE_SPEC> {
        LP_REG_SOFT_RESET_W::new(self, 27)
    }
}
#[doc = "SPI slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave::R`](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave::W`](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE to value 0"]
impl crate::Resettable for SLAVE_SPEC {}
