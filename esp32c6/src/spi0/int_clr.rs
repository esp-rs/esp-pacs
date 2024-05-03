#[doc = "Register `INT_CLR` reader"]
pub type R = crate::R<INT_CLR_SPEC>;
#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLV_ST_END` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_ST_END` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ECC_ERR` reader - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_R = crate::BitReader;
#[doc = "Field `PMS_REJECT` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` reader - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR` reader - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AXI_WR_FLASH_ERR_R {
        AXI_WR_FLASH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err(&self) -> AXI_WADDR_ERR_R {
        AXI_WADDR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR")
            .field("ecc_err", &self.ecc_err().bit())
            .field("axi_wr_flash_err", &self.axi_wr_flash_err().bit())
            .field("axi_waddr_err", &self.axi_waddr_err().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<INT_CLR_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<INT_CLR_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<INT_CLR_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<INT_CLR_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
}
#[doc = "SPI0 interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clr::R`](R) reader structure"]
impl crate::Readable for INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xd8;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
