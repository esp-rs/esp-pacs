#[doc = "Register `CACHE_FCTRL` reader"]
pub type R = crate::R<CACHE_FCTRL_SPEC>;
#[doc = "Register `CACHE_FCTRL` writer"]
pub type W = crate::W<CACHE_FCTRL_SPEC>;
#[doc = "Field `CACHE_REQ_EN` reader - For SPI0, Cache access enable, 1: enable, 0:disable."]
pub type CACHE_REQ_EN_R = crate::BitReader;
#[doc = "Field `CACHE_REQ_EN` writer - For SPI0, Cache access enable, 1: enable, 0:disable."]
pub type CACHE_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` reader - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type CACHE_USR_ADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` writer - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type CACHE_USR_ADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_DUAL` reader - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `FDIN_DUAL` writer - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_DUAL` reader - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `FDOUT_DUAL` writer - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_DUAL` reader - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `FADDR_DUAL` writer - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_QUAD` reader - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `FDIN_QUAD` writer - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_QUAD` reader - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `FDOUT_QUAD` writer - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_QUAD` reader - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `FADDR_QUAD` writer - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0, Cache access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&self) -> CACHE_USR_ADDR_4BYTE_R {
        CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FCTRL")
            .field(
                "cache_req_en",
                &format_args!("{}", self.cache_req_en().bit()),
            )
            .field(
                "cache_usr_addr_4byte",
                &format_args!("{}", self.cache_usr_addr_4byte().bit()),
            )
            .field(
                "cache_flash_usr_cmd",
                &format_args!("{}", self.cache_flash_usr_cmd().bit()),
            )
            .field("fdin_dual", &format_args!("{}", self.fdin_dual().bit()))
            .field("fdout_dual", &format_args!("{}", self.fdout_dual().bit()))
            .field("faddr_dual", &format_args!("{}", self.faddr_dual().bit()))
            .field("fdin_quad", &format_args!("{}", self.fdin_quad().bit()))
            .field("fdout_quad", &format_args!("{}", self.fdout_quad().bit()))
            .field("faddr_quad", &format_args!("{}", self.faddr_quad().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_FCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, Cache access enable, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W<CACHE_FCTRL_SPEC> {
        CACHE_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn cache_usr_addr_4byte(&mut self) -> CACHE_USR_ADDR_4BYTE_W<CACHE_FCTRL_SPEC> {
        CACHE_USR_ADDR_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<CACHE_FCTRL_SPEC> {
        CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W<CACHE_FCTRL_SPEC> {
        FDIN_DUAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W<CACHE_FCTRL_SPEC> {
        FDOUT_DUAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W<CACHE_FCTRL_SPEC> {
        FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W<CACHE_FCTRL_SPEC> {
        FDIN_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W<CACHE_FCTRL_SPEC> {
        FDOUT_QUAD_W::new(self, 7)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W<CACHE_FCTRL_SPEC> {
        FADDR_QUAD_W::new(self, 8)
    }
}
#[doc = "SPI0 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_fctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_fctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
