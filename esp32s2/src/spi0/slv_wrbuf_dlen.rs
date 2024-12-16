#[doc = "Register `SLV_WRBUF_DLEN` reader"]
pub type R = crate::R<SLV_WRBUF_DLEN_SPEC>;
#[doc = "Register `SLV_WRBUF_DLEN` writer"]
pub type W = crate::W<SLV_WRBUF_DLEN_SPEC>;
#[doc = "Field `SLV_WR_BUF_DONE` reader - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE` writer - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
pub type SLV_WR_BUF_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_BASE_BITLEN` reader - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub type CONF_BASE_BITLEN_R = crate::FieldReader;
#[doc = "Field `CONF_BASE_BITLEN` writer - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
pub type CONF_BASE_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    pub fn conf_base_bitlen(&self) -> CONF_BASE_BITLEN_R {
        CONF_BASE_BITLEN_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_WRBUF_DLEN")
            .field("slv_wr_buf_done", &self.slv_wr_buf_done())
            .field("conf_base_bitlen", &self.conf_base_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - The interrupt raw bit for the completion of write-buffer operation in the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W<SLV_WRBUF_DLEN_SPEC> {
        SLV_WR_BUF_DONE_W::new(self, 24)
    }
    #[doc = "Bits 25:31 - The basic spi_clk cycles of CONF state. The real cycle length of CONF state, if SPI_USR_CONF is enabled, is SPI_CONF_BASE_BITLEN\\[6:0\\] + SPI_CONF_BITLEN\\[23:0\\]."]
    #[inline(always)]
    pub fn conf_base_bitlen(&mut self) -> CONF_BASE_BITLEN_W<SLV_WRBUF_DLEN_SPEC> {
        CONF_BASE_BITLEN_W::new(self, 25)
    }
}
#[doc = "SPI slave Wr_BUF interrupt and CONF control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_wrbuf_dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_WRBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_WRBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_wrbuf_dlen::R`](R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_wrbuf_dlen::W`](W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLV_WRBUF_DLEN to value 0xd800_0000"]
impl crate::Resettable for SLV_WRBUF_DLEN_SPEC {
    const RESET_VALUE: u32 = 0xd800_0000;
}
