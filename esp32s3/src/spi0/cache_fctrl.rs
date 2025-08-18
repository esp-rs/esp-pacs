#[doc = "Register `CACHE_FCTRL` reader"]
pub type R = crate::R<CACHE_FCTRL_SPEC>;
#[doc = "Register `CACHE_FCTRL` writer"]
pub type W = crate::W<CACHE_FCTRL_SPEC>;
#[doc = "Field `CACHE_REQ_EN` reader - Set this bit to enable Cache's access and SPI0's transfer."]
pub type CACHE_REQ_EN_R = crate::BitReader;
#[doc = "Field `CACHE_REQ_EN` writer - Set this bit to enable Cache's access and SPI0's transfer."]
pub type CACHE_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_USR_CMD_4BYTE` reader - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub type CACHE_USR_CMD_4BYTE_R = crate::BitReader;
#[doc = "Field `CACHE_USR_CMD_4BYTE` writer - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub type CACHE_USR_CMD_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
pub type CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
pub type FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `FDIN_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
pub type FDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
pub type FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `FDOUT_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
pub type FDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
pub type FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `FADDR_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
pub type FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
pub type FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `FDIN_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
pub type FDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
pub type FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `FDOUT_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
pub type FDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
pub type FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `FADDR_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
pub type FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable Cache's access and SPI0's transfer."]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FCTRL")
            .field("cache_req_en", &self.cache_req_en())
            .field("cache_usr_cmd_4byte", &self.cache_usr_cmd_4byte())
            .field("cache_flash_usr_cmd", &self.cache_flash_usr_cmd())
            .field("fdin_dual", &self.fdin_dual())
            .field("fdout_dual", &self.fdout_dual())
            .field("faddr_dual", &self.faddr_dual())
            .field("fdin_quad", &self.fdin_quad())
            .field("fdout_quad", &self.fdout_quad())
            .field("faddr_quad", &self.faddr_quad())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable Cache's access and SPI0's transfer."]
    #[inline(always)]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W<'_, CACHE_FCTRL_SPEC> {
        CACHE_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W<'_, CACHE_FCTRL_SPEC> {
        CACHE_USR_CMD_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<'_, CACHE_FCTRL_SPEC> {
        CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    #[doc = "Bit 3 - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FDIN_DUAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FDOUT_DUAL_W::new(self, 4)
    }
    #[doc = "Bit 5 - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W<'_, CACHE_FCTRL_SPEC> {
        FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FDIN_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7 - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FDOUT_QUAD_W::new(self, 7)
    }
    #[doc = "Bit 8 - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W<'_, CACHE_FCTRL_SPEC> {
        FADDR_QUAD_W::new(self, 8)
    }
}
#[doc = "SPI0 external RAM bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {}
