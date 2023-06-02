#[doc = "Register `SPI_MEM_INT_CLR` reader"]
pub struct R(crate::R<SPI_MEM_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_INT_CLR` writer"]
pub struct W(crate::W<SPI_MEM_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_INT_CLR_SPEC>;
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
impl From<crate::W<SPI_MEM_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_CLR_SPEC, O>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_CLR_SPEC, O>;
#[doc = "Field `SPI_MEM_ECC_ERR_INT_CLR` reader - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type SPI_MEM_ECC_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_CLR` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type SPI_MEM_PMS_REJECT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_CLR_SPEC, O>;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_CLR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_INT_CLR_SPEC, O>;
#[doc = "Field `SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR` reader - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WADDR_ERR_INT_CLR` reader - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WADDR_ERR_INT_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_clr(&self) -> SPI_MEM_ECC_ERR_INT_CLR_R {
        SPI_MEM_ECC_ERR_INT_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_clr(&self) -> SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_R {
        SPI_MEM_AXI_WR_FLASH_ERR_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_clr(&self) -> SPI_MEM_AXI_WADDR_ERR_INT_CLR_R {
        SPI_MEM_AXI_WADDR_ERR_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_CLR")
            .field(
                "spi_mem_ecc_err_int_clr",
                &format_args!("{}", self.spi_mem_ecc_err_int_clr().bit()),
            )
            .field(
                "spi_mem_axi_wr_flash_err_int_clr",
                &format_args!("{}", self.spi_mem_axi_wr_flash_err_int_clr().bit()),
            )
            .field(
                "spi_mem_axi_waddr_err_int_clr",
                &format_args!("{}", self.spi_mem_axi_waddr_err_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_slv_st_end_int_clr(&mut self) -> SPI_MEM_SLV_ST_END_INT_CLR_W<3> {
        SPI_MEM_SLV_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_mst_st_end_int_clr(&mut self) -> SPI_MEM_MST_ST_END_INT_CLR_W<4> {
        SPI_MEM_MST_ST_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pms_reject_int_clr(&mut self) -> SPI_MEM_PMS_REJECT_INT_CLR_W<6> {
        SPI_MEM_PMS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_raddr_err_int_clr(&mut self) -> SPI_MEM_AXI_RADDR_ERR_INT_CLR_W<7> {
        SPI_MEM_AXI_RADDR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_int_clr](index.html) module"]
pub struct SPI_MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_int_clr::R](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_int_clr::W](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_INT_CLR to value 0"]
impl crate::Resettable for SPI_MEM_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
