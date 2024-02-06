#[doc = "Register `SPI_MEM_INT_ENA` reader"]
pub type R = crate::R<SPI_MEM_INT_ENA_SPEC>;
#[doc = "Register `SPI_MEM_INT_ENA` writer"]
pub type W = crate::W<SPI_MEM_INT_ENA_SPEC>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SPI_MEM_SLV_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type SPI_MEM_MST_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_ECC_ERR_INT_ENA` reader - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type SPI_MEM_ECC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_ENA` reader - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type SPI_MEM_PMS_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_ENA` writer - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type SPI_MEM_PMS_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_ENA` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_ENA` writer - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WADDR_ERR_INT__ENA` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type SPI_MEM_AXI_WADDR_ERR_INT__ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_ena(&self) -> SPI_MEM_SLV_ST_END_INT_ENA_R {
        SPI_MEM_SLV_ST_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_ena(&self) -> SPI_MEM_MST_ST_END_INT_ENA_R {
        SPI_MEM_MST_ST_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_ena(&self) -> SPI_MEM_ECC_ERR_INT_ENA_R {
        SPI_MEM_ECC_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_ena(&self) -> SPI_MEM_PMS_REJECT_INT_ENA_R {
        SPI_MEM_PMS_REJECT_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_ena(&self) -> SPI_MEM_AXI_RADDR_ERR_INT_ENA_R {
        SPI_MEM_AXI_RADDR_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_ena(&self) -> SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_R {
        SPI_MEM_AXI_WR_FLASH_ERR_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int__ena(&self) -> SPI_MEM_AXI_WADDR_ERR_INT__ENA_R {
        SPI_MEM_AXI_WADDR_ERR_INT__ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_ENA")
            .field(
                "spi_mem_slv_st_end_int_ena",
                &format_args!("{}", self.spi_mem_slv_st_end_int_ena().bit()),
            )
            .field(
                "spi_mem_mst_st_end_int_ena",
                &format_args!("{}", self.spi_mem_mst_st_end_int_ena().bit()),
            )
            .field(
                "spi_mem_ecc_err_int_ena",
                &format_args!("{}", self.spi_mem_ecc_err_int_ena().bit()),
            )
            .field(
                "spi_mem_pms_reject_int_ena",
                &format_args!("{}", self.spi_mem_pms_reject_int_ena().bit()),
            )
            .field(
                "spi_mem_axi_raddr_err_int_ena",
                &format_args!("{}", self.spi_mem_axi_raddr_err_int_ena().bit()),
            )
            .field(
                "spi_mem_axi_wr_flash_err_int_ena",
                &format_args!("{}", self.spi_mem_axi_wr_flash_err_int_ena().bit()),
            )
            .field(
                "spi_mem_axi_waddr_err_int__ena",
                &format_args!("{}", self.spi_mem_axi_waddr_err_int__ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_slv_st_end_int_ena(
        &mut self,
    ) -> SPI_MEM_SLV_ST_END_INT_ENA_W<SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_SLV_ST_END_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_mst_st_end_int_ena(
        &mut self,
    ) -> SPI_MEM_MST_ST_END_INT_ENA_W<SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_MST_ST_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pms_reject_int_ena(
        &mut self,
    ) -> SPI_MEM_PMS_REJECT_INT_ENA_W<SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_PMS_REJECT_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_raddr_err_int_ena(
        &mut self,
    ) -> SPI_MEM_AXI_RADDR_ERR_INT_ENA_W<SPI_MEM_INT_ENA_SPEC> {
        SPI_MEM_AXI_RADDR_ERR_INT_ENA_W::new(self, 7)
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
#[doc = "SPI0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_ENA_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_int_ena::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_int_ena::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_INT_ENA to value 0"]
impl crate::Resettable for SPI_MEM_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
