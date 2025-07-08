#[doc = "Register `MEM_CACHE_FCTRL` reader"]
pub type R = crate::R<MEM_CACHE_FCTRL_SPEC>;
#[doc = "Register `MEM_CACHE_FCTRL` writer"]
pub type W = crate::W<MEM_CACHE_FCTRL_SPEC>;
#[doc = "Field `MEM_AXI_REQ_EN` reader - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type MEM_AXI_REQ_EN_R = crate::BitReader;
#[doc = "Field `MEM_AXI_REQ_EN` writer - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type MEM_AXI_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CACHE_USR_ADDR_4BYTE` reader - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type MEM_CACHE_USR_ADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `MEM_CACHE_USR_ADDR_4BYTE` writer - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type MEM_CACHE_USR_ADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CACHE_FLASH_USR_CMD` reader - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type MEM_CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `MEM_CACHE_FLASH_USR_CMD` writer - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type MEM_CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FDIN_DUAL` reader - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_FDIN_DUAL` writer - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FDOUT_DUAL` reader - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_FDOUT_DUAL` writer - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FADDR_DUAL` reader - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `MEM_FADDR_DUAL` writer - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type MEM_FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FDIN_QUAD` reader - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_FDIN_QUAD` writer - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FDOUT_QUAD` reader - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_FDOUT_QUAD` writer - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FADDR_QUAD` reader - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `MEM_FADDR_QUAD` writer - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type MEM_FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAME_AW_AR_ADDR_CHK_EN` reader - Set this bit to check AXI read/write the same address region."]
pub type SAME_AW_AR_ADDR_CHK_EN_R = crate::BitReader;
#[doc = "Field `SAME_AW_AR_ADDR_CHK_EN` writer - Set this bit to check AXI read/write the same address region."]
pub type SAME_AW_AR_ADDR_CHK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOSE_AXI_INF_EN` reader - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type CLOSE_AXI_INF_EN_R = crate::BitReader;
#[doc = "Field `CLOSE_AXI_INF_EN` writer - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type CLOSE_AXI_INF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_axi_req_en(&self) -> MEM_AXI_REQ_EN_R {
        MEM_AXI_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_usr_addr_4byte(&self) -> MEM_CACHE_USR_ADDR_4BYTE_R {
        MEM_CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_flash_usr_cmd(&self) -> MEM_CACHE_FLASH_USR_CMD_R {
        MEM_CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_fdin_dual(&self) -> MEM_FDIN_DUAL_R {
        MEM_FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_fdout_dual(&self) -> MEM_FDOUT_DUAL_R {
        MEM_FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_faddr_dual(&self) -> MEM_FADDR_DUAL_R {
        MEM_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_fdin_quad(&self) -> MEM_FDIN_QUAD_R {
        MEM_FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_fdout_quad(&self) -> MEM_FDOUT_QUAD_R {
        MEM_FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_faddr_quad(&self) -> MEM_FADDR_QUAD_R {
        MEM_FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to check AXI read/write the same address region."]
    #[inline(always)]
    pub fn same_aw_ar_addr_chk_en(&self) -> SAME_AW_AR_ADDR_CHK_EN_R {
        SAME_AW_AR_ADDR_CHK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    pub fn close_axi_inf_en(&self) -> CLOSE_AXI_INF_EN_R {
        CLOSE_AXI_INF_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CACHE_FCTRL")
            .field("mem_axi_req_en", &self.mem_axi_req_en())
            .field("mem_cache_usr_addr_4byte", &self.mem_cache_usr_addr_4byte())
            .field("mem_cache_flash_usr_cmd", &self.mem_cache_flash_usr_cmd())
            .field("mem_fdin_dual", &self.mem_fdin_dual())
            .field("mem_fdout_dual", &self.mem_fdout_dual())
            .field("mem_faddr_dual", &self.mem_faddr_dual())
            .field("mem_fdin_quad", &self.mem_fdin_quad())
            .field("mem_fdout_quad", &self.mem_fdout_quad())
            .field("mem_faddr_quad", &self.mem_faddr_quad())
            .field("same_aw_ar_addr_chk_en", &self.same_aw_ar_addr_chk_en())
            .field("close_axi_inf_en", &self.close_axi_inf_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_axi_req_en(&mut self) -> MEM_AXI_REQ_EN_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_AXI_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_usr_addr_4byte(&mut self) -> MEM_CACHE_USR_ADDR_4BYTE_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_CACHE_USR_ADDR_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_flash_usr_cmd(&mut self) -> MEM_CACHE_FLASH_USR_CMD_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_fdin_dual(&mut self) -> MEM_FDIN_DUAL_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FDIN_DUAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_fdout_dual(&mut self) -> MEM_FDOUT_DUAL_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FDOUT_DUAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn mem_faddr_dual(&mut self) -> MEM_FADDR_DUAL_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_fdin_quad(&mut self) -> MEM_FDIN_QUAD_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FDIN_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_fdout_quad(&mut self) -> MEM_FDOUT_QUAD_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FDOUT_QUAD_W::new(self, 7)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn mem_faddr_quad(&mut self) -> MEM_FADDR_QUAD_W<MEM_CACHE_FCTRL_SPEC> {
        MEM_FADDR_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 30 - Set this bit to check AXI read/write the same address region."]
    #[inline(always)]
    pub fn same_aw_ar_addr_chk_en(&mut self) -> SAME_AW_AR_ADDR_CHK_EN_W<MEM_CACHE_FCTRL_SPEC> {
        SAME_AW_AR_ADDR_CHK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    pub fn close_axi_inf_en(&mut self) -> CLOSE_AXI_INF_EN_W<MEM_CACHE_FCTRL_SPEC> {
        CLOSE_AXI_INF_EN_W::new(self, 31)
    }
}
#[doc = "SPI0 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cache_fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_cache_fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for MEM_CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_cache_fctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CACHE_FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_cache_fctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CACHE_FCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CACHE_FCTRL to value 0xc000_0000"]
impl crate::Resettable for MEM_CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0xc000_0000;
}
