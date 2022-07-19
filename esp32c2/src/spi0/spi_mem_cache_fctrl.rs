#[doc = "Register `SPI_MEM_CACHE_FCTRL` reader"]
pub struct R(crate::R<SPI_MEM_CACHE_FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CACHE_FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CACHE_FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CACHE_FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CACHE_FCTRL` writer"]
pub struct W(crate::W<SPI_MEM_CACHE_FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CACHE_FCTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_CACHE_FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CACHE_FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CACHE_REQ_EN` reader - For SPI0, Cache access enable, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_REQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CACHE_REQ_EN` writer - For SPI0, Cache access enable, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_REQ_EN_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 0>;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` reader - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` writer - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_CACHE_FLASH_USR_CMD` reader - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_FLASH_USR_CMD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CACHE_FLASH_USR_CMD` writer - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_FLASH_USR_CMD_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 2>;
#[doc = "Field `SPI_MEM_FDIN_DUAL` reader - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FDIN_DUAL` writer - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 3>;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` reader - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` writer - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 4>;
#[doc = "Field `SPI_MEM_FADDR_DUAL` reader - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FADDR_DUAL` writer - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 5>;
#[doc = "Field `SPI_MEM_FDIN_QUAD` reader - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FDIN_QUAD` writer - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 6>;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` reader - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` writer - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 7>;
#[doc = "Field `SPI_MEM_FADDR_QUAD` reader - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_FADDR_QUAD` writer - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CACHE_FCTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - For SPI0, Cache access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_req_en(&self) -> SPI_MEM_CACHE_REQ_EN_R {
        SPI_MEM_CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_R {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&self) -> SPI_MEM_CACHE_FLASH_USR_CMD_R {
        SPI_MEM_CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&self) -> SPI_MEM_FDIN_DUAL_R {
        SPI_MEM_FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&self) -> SPI_MEM_FDOUT_DUAL_R {
        SPI_MEM_FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&self) -> SPI_MEM_FADDR_DUAL_R {
        SPI_MEM_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&self) -> SPI_MEM_FDIN_QUAD_R {
        SPI_MEM_FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&self) -> SPI_MEM_FDOUT_QUAD_R {
        SPI_MEM_FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&self) -> SPI_MEM_FADDR_QUAD_R {
        SPI_MEM_FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, Cache access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_req_en(&mut self) -> SPI_MEM_CACHE_REQ_EN_W {
        SPI_MEM_CACHE_REQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&mut self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_W {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_W::new(self)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&mut self) -> SPI_MEM_CACHE_FLASH_USR_CMD_W {
        SPI_MEM_CACHE_FLASH_USR_CMD_W::new(self)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&mut self) -> SPI_MEM_FDIN_DUAL_W {
        SPI_MEM_FDIN_DUAL_W::new(self)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&mut self) -> SPI_MEM_FDOUT_DUAL_W {
        SPI_MEM_FDOUT_DUAL_W::new(self)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&mut self) -> SPI_MEM_FADDR_DUAL_W {
        SPI_MEM_FADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&mut self) -> SPI_MEM_FDIN_QUAD_W {
        SPI_MEM_FDIN_QUAD_W::new(self)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&mut self) -> SPI_MEM_FDOUT_QUAD_W {
        SPI_MEM_FDOUT_QUAD_W::new(self)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&mut self) -> SPI_MEM_FADDR_QUAD_W {
        SPI_MEM_FADDR_QUAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 bit mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cache_fctrl](index.html) module"]
pub struct SPI_MEM_CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_cache_fctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CACHE_FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_cache_fctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CACHE_FCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_CACHE_FCTRL to value 0"]
impl crate::Resettable for SPI_MEM_CACHE_FCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
