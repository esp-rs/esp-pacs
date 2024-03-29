#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLV_ST_END` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_R = crate::BitReader;
#[doc = "Field `SLV_ST_END` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_R = crate::BitReader;
#[doc = "Field `MST_ST_END` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` reader - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_R = crate::BitReader;
#[doc = "Field `PMS_REJECT` reader - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_R = crate::BitReader;
#[doc = "Field `PMS_REJECT` writer - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR` writer - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR_INT__ENA` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_INT__ENA_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SLV_ST_END_R {
        SLV_ST_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MST_ST_END_R {
        MST_ST_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject(&self) -> PMS_REJECT_R {
        PMS_REJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err(&self) -> AXI_RADDR_ERR_R {
        AXI_RADDR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AXI_WR_FLASH_ERR_R {
        AXI_WR_FLASH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err_int__ena(&self) -> AXI_WADDR_ERR_INT__ENA_R {
        AXI_WADDR_ERR_INT__ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("slv_st_end", &format_args!("{}", self.slv_st_end().bit()))
            .field("mst_st_end", &format_args!("{}", self.mst_st_end().bit()))
            .field("ecc_err", &format_args!("{}", self.ecc_err().bit()))
            .field("pms_reject", &format_args!("{}", self.pms_reject().bit()))
            .field(
                "axi_raddr_err",
                &format_args!("{}", self.axi_raddr_err().bit()),
            )
            .field(
                "axi_wr_flash_err",
                &format_args!("{}", self.axi_wr_flash_err().bit()),
            )
            .field(
                "axi_waddr_err_int__ena",
                &format_args!("{}", self.axi_waddr_err_int__ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<INT_ENA_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<INT_ENA_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<INT_ENA_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<INT_ENA_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
}
#[doc = "SPI0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
