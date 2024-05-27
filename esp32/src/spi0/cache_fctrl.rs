///Register `CACHE_FCTRL` reader
pub type R = crate::R<CACHE_FCTRL_SPEC>;
///Register `CACHE_FCTRL` writer
pub type W = crate::W<CACHE_FCTRL_SPEC>;
///Field `CACHE_REQ_EN` reader - For SPI0 Cache access enable 1: enable 0:disable.
pub type CACHE_REQ_EN_R = crate::BitReader;
///Field `CACHE_REQ_EN` writer - For SPI0 Cache access enable 1: enable 0:disable.
pub type CACHE_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_USR_CMD_4BYTE` reader - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable.
pub type CACHE_USR_CMD_4BYTE_R = crate::BitReader;
///Field `CACHE_USR_CMD_4BYTE` writer - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable.
pub type CACHE_USR_CMD_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_FLASH_USR_CMD` reader - For SPI0 cache read flash for user define command 1: enable 0:disable.
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader;
///Field `CACHE_FLASH_USR_CMD` writer - For SPI0 cache read flash for user define command 1: enable 0:disable.
pub type CACHE_FLASH_USR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_FLASH_PES_EN` reader - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable.
pub type CACHE_FLASH_PES_EN_R = crate::BitReader;
///Field `CACHE_FLASH_PES_EN` writer - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable.
pub type CACHE_FLASH_PES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - For SPI0 Cache access enable 1: enable 0:disable.
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable.
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable.
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable.
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
    ///Bit 0 - For SPI0 Cache access enable 1: enable 0:disable.
    #[inline(always)]
    #[must_use]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W<CACHE_FCTRL_SPEC> {
        CACHE_REQ_EN_W::new(self, 0)
    }
    ///Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable.
    #[inline(always)]
    #[must_use]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W<CACHE_FCTRL_SPEC> {
        CACHE_USR_CMD_4BYTE_W::new(self, 1)
    }
    ///Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable.
    #[inline(always)]
    #[must_use]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<CACHE_FCTRL_SPEC> {
        CACHE_FLASH_USR_CMD_W::new(self, 2)
    }
    ///Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable.
    #[inline(always)]
    #[must_use]
    pub fn cache_flash_pes_en(&mut self) -> CACHE_FLASH_PES_EN_W<CACHE_FCTRL_SPEC> {
        CACHE_FLASH_PES_EN_W::new(self, 3)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_fctrl::R`](R) reader structure
impl crate::Readable for CACHE_FCTRL_SPEC {}
///`write(|w| ..)` method takes [`cache_fctrl::W`](W) writer structure
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_FCTRL to value 0
impl crate::Resettable for CACHE_FCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
