#[doc = "Register `CACHE_FCTRL` reader"]
pub type R = crate::R<CACHE_FCTRL_SPEC>;
#[doc = "Register `CACHE_FCTRL` writer"]
pub type W = crate::W<CACHE_FCTRL_SPEC>;
#[doc = "Field `CACHE_REQ_EN` reader - For SPI0 Cache access enable 1: enable 0:disable."]
pub type CACHE_REQ_EN_R = crate::BitReader;
#[doc = "Field `CACHE_REQ_EN` writer - For SPI0 Cache access enable 1: enable 0:disable."]
pub type CACHE_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_USR_CMD_4BYTE` reader - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
pub type CACHE_USR_CMD_4BYTE_R = crate::BitReader;
#[doc = "Field `CACHE_USR_CMD_4BYTE` writer - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
pub type CACHE_USR_CMD_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - For SPI0 cache read flash for user define command 1: enable 0:disable."]
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - For SPI0 cache read flash for user define command 1: enable 0:disable."]
pub type CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_PES_EN` reader - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
pub type CACHE_FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `CACHE_FLASH_PES_EN` writer - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
pub type CACHE_FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_pes_en(&self) -> CACHE_FLASH_PES_EN_R {
        CACHE_FLASH_PES_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FCTRL")
            .field("cache_req_en", &self.cache_req_en())
            .field("cache_usr_cmd_4byte", &self.cache_usr_cmd_4byte())
            .field("cache_flash_usr_cmd", &self.cache_flash_usr_cmd())
            .field("cache_flash_pes_en", &self.cache_flash_pes_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W<CACHE_FCTRL_SPEC> {
        CACHE_REQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W<CACHE_FCTRL_SPEC> {
        CACHE_USR_CMD_4BYTE_W::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<CACHE_FCTRL_SPEC> {
        CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_pes_en(&mut self) -> CACHE_FLASH_PES_EN_W<CACHE_FCTRL_SPEC> {
        CACHE_FLASH_PES_EN_W::new(self, 3)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
