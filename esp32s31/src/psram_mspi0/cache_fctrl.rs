#[doc = "Register `CACHE_FCTRL` reader"]
pub type R = crate::R<CACHE_FCTRL_SPEC>;
#[doc = "Register `CACHE_FCTRL` writer"]
pub type W = crate::W<CACHE_FCTRL_SPEC>;
#[doc = "Field `AXI_REQ_EN` reader - "]
pub type AXI_REQ_EN_R = crate::BitReader;
#[doc = "Field `AXI_REQ_EN` writer - "]
pub type AXI_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` reader - "]
pub type CACHE_USR_ADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` writer - "]
pub type CACHE_USR_ADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - "]
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - "]
pub type CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_DUAL` reader - "]
pub type FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `FDIN_DUAL` writer - "]
pub type FDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_DUAL` reader - "]
pub type FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `FDOUT_DUAL` writer - "]
pub type FDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_DUAL` reader - "]
pub type FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `FADDR_DUAL` writer - "]
pub type FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_QUAD` reader - "]
pub type FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `FDIN_QUAD` writer - "]
pub type FDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_QUAD` reader - "]
pub type FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `FDOUT_QUAD` writer - "]
pub type FDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_QUAD` reader - "]
pub type FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `FADDR_QUAD` writer - "]
pub type FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_WEI_EN` reader - "]
pub type ARB_WEI_EN_R = crate::BitReader;
#[doc = "Field `ARB_WEI_EN` writer - "]
pub type ARB_WEI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ0_PRI` reader - "]
pub type ARB_REQ0_PRI_R = crate::BitReader;
#[doc = "Field `ARB_REQ0_PRI` writer - "]
pub type ARB_REQ0_PRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ1_PRI` reader - "]
pub type ARB_REQ1_PRI_R = crate::BitReader;
#[doc = "Field `ARB_REQ1_PRI` writer - "]
pub type ARB_REQ1_PRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ0_WEI` reader - "]
pub type ARB_REQ0_WEI_R = crate::FieldReader;
#[doc = "Field `ARB_REQ0_WEI` writer - "]
pub type ARB_REQ0_WEI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARB_REQ1_WEI` reader - "]
pub type ARB_REQ1_WEI_R = crate::FieldReader;
#[doc = "Field `ARB_REQ1_WEI` writer - "]
pub type ARB_REQ1_WEI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI_SAME_AW_AR_ADDR_CHK_EN` reader - "]
pub type SPI_SAME_AW_AR_ADDR_CHK_EN_R = crate::BitReader;
#[doc = "Field `SPI_SAME_AW_AR_ADDR_CHK_EN` writer - "]
pub type SPI_SAME_AW_AR_ADDR_CHK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` reader - "]
pub type SPI_CLOSE_AXI_INF_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` writer - "]
pub type SPI_CLOSE_AXI_INF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn axi_req_en(&self) -> AXI_REQ_EN_R {
        AXI_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&self) -> CACHE_USR_ADDR_4BYTE_R {
        CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn arb_wei_en(&self) -> ARB_WEI_EN_R {
        ARB_WEI_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn arb_req0_pri(&self) -> ARB_REQ0_PRI_R {
        ARB_REQ0_PRI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn arb_req1_pri(&self) -> ARB_REQ1_PRI_R {
        ARB_REQ1_PRI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn arb_req0_wei(&self) -> ARB_REQ0_WEI_R {
        ARB_REQ0_WEI_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn arb_req1_wei(&self) -> ARB_REQ1_WEI_R {
        ARB_REQ1_WEI_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(&self) -> SPI_SAME_AW_AR_ADDR_CHK_EN_R {
        SPI_SAME_AW_AR_ADDR_CHK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&self) -> SPI_CLOSE_AXI_INF_EN_R {
        SPI_CLOSE_AXI_INF_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FCTRL")
            .field("axi_req_en", &self.axi_req_en())
            .field("cache_usr_addr_4byte", &self.cache_usr_addr_4byte())
            .field("cache_flash_usr_cmd", &self.cache_flash_usr_cmd())
            .field("fdin_dual", &self.fdin_dual())
            .field("fdout_dual", &self.fdout_dual())
            .field("faddr_dual", &self.faddr_dual())
            .field("fdin_quad", &self.fdin_quad())
            .field("fdout_quad", &self.fdout_quad())
            .field("faddr_quad", &self.faddr_quad())
            .field("arb_wei_en", &self.arb_wei_en())
            .field("arb_req0_pri", &self.arb_req0_pri())
            .field("arb_req1_pri", &self.arb_req1_pri())
            .field("arb_req0_wei", &self.arb_req0_wei())
            .field("arb_req1_wei", &self.arb_req1_wei())
            .field(
                "spi_same_aw_ar_addr_chk_en",
                &self.spi_same_aw_ar_addr_chk_en(),
            )
            .field("spi_close_axi_inf_en", &self.spi_close_axi_inf_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn axi_req_en(&mut self) -> AXI_REQ_EN_W<'_, CACHE_FCTRL_SPEC> {
        AXI_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&mut self) -> CACHE_USR_ADDR_4BYTE_W<'_, CACHE_FCTRL_SPEC> {
        CACHE_USR_ADDR_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<'_, CACHE_FCTRL_SPEC> {
        CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FDIN_DUAL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FDOUT_DUAL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FDIN_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FDOUT_QUAD_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FADDR_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn arb_wei_en(&mut self) -> ARB_WEI_EN_W<'_, CACHE_FCTRL_SPEC> {
        ARB_WEI_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn arb_req0_pri(&mut self) -> ARB_REQ0_PRI_W<'_, CACHE_FCTRL_SPEC> {
        ARB_REQ0_PRI_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn arb_req1_pri(&mut self) -> ARB_REQ1_PRI_W<'_, CACHE_FCTRL_SPEC> {
        ARB_REQ1_PRI_W::new(self, 11)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn arb_req0_wei(&mut self) -> ARB_REQ0_WEI_W<'_, CACHE_FCTRL_SPEC> {
        ARB_REQ0_WEI_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn arb_req1_wei(&mut self) -> ARB_REQ1_WEI_W<'_, CACHE_FCTRL_SPEC> {
        ARB_REQ1_WEI_W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(
        &mut self,
    ) -> SPI_SAME_AW_AR_ADDR_CHK_EN_W<'_, CACHE_FCTRL_SPEC> {
        SPI_SAME_AW_AR_ADDR_CHK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&mut self) -> SPI_CLOSE_AXI_INF_EN_W<'_, CACHE_FCTRL_SPEC> {
        SPI_CLOSE_AXI_INF_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_fctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_fctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0xc000_0000"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0xc000_0000;
}
