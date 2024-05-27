///Register `FLASH_WAITI_CTRL` reader
pub type R = crate::R<FLASH_WAITI_CTRL_SPEC>;
///Register `FLASH_WAITI_CTRL` writer
pub type W = crate::W<FLASH_WAITI_CTRL_SPEC>;
///Field `WAITI_EN` reader - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent.
pub type WAITI_EN_R = crate::BitReader;
///Field `WAITI_EN` writer - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent.
pub type WAITI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITI_DUMMY` reader - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR).
pub type WAITI_DUMMY_R = crate::BitReader;
///Field `WAITI_DUMMY` writer - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR).
pub type WAITI_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITI_CMD` reader - The command value of auto wait flash idle transfer(RDSR).
pub type WAITI_CMD_R = crate::FieldReader;
///Field `WAITI_CMD` writer - The command value of auto wait flash idle transfer(RDSR).
pub type WAITI_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WAITI_DUMMY_CYCLELEN` reader - The dummy cycle length when wait flash idle(RDSR).
pub type WAITI_DUMMY_CYCLELEN_R = crate::FieldReader;
///Field `WAITI_DUMMY_CYCLELEN` writer - The dummy cycle length when wait flash idle(RDSR).
pub type WAITI_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent.
    #[inline(always)]
    pub fn waiti_en(&self) -> WAITI_EN_R {
        WAITI_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR).
    #[inline(always)]
    pub fn waiti_dummy(&self) -> WAITI_DUMMY_R {
        WAITI_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:9 - The command value of auto wait flash idle transfer(RDSR).
    #[inline(always)]
    pub fn waiti_cmd(&self) -> WAITI_CMD_R {
        WAITI_CMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    ///Bits 10:15 - The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    pub fn waiti_dummy_cyclelen(&self) -> WAITI_DUMMY_CYCLELEN_R {
        WAITI_DUMMY_CYCLELEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_WAITI_CTRL")
            .field("waiti_en", &self.waiti_en())
            .field("waiti_dummy", &self.waiti_dummy())
            .field("waiti_cmd", &self.waiti_cmd())
            .field("waiti_dummy_cyclelen", &self.waiti_dummy_cyclelen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent.
    #[inline(always)]
    #[must_use]
    pub fn waiti_en(&mut self) -> WAITI_EN_W<FLASH_WAITI_CTRL_SPEC> {
        WAITI_EN_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR).
    #[inline(always)]
    #[must_use]
    pub fn waiti_dummy(&mut self) -> WAITI_DUMMY_W<FLASH_WAITI_CTRL_SPEC> {
        WAITI_DUMMY_W::new(self, 1)
    }
    ///Bits 2:9 - The command value of auto wait flash idle transfer(RDSR).
    #[inline(always)]
    #[must_use]
    pub fn waiti_cmd(&mut self) -> WAITI_CMD_W<FLASH_WAITI_CTRL_SPEC> {
        WAITI_CMD_W::new(self, 2)
    }
    ///Bits 10:15 - The dummy cycle length when wait flash idle(RDSR).
    #[inline(always)]
    #[must_use]
    pub fn waiti_dummy_cyclelen(&mut self) -> WAITI_DUMMY_CYCLELEN_W<FLASH_WAITI_CTRL_SPEC> {
        WAITI_DUMMY_CYCLELEN_W::new(self, 10)
    }
}
/**SPI1 wait idle control register

You can [`read`](crate::generic::Reg::read) this register and get [`flash_waiti_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLASH_WAITI_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_WAITI_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`flash_waiti_ctrl::R`](R) reader structure
impl crate::Readable for FLASH_WAITI_CTRL_SPEC {}
///`write(|w| ..)` method takes [`flash_waiti_ctrl::W`](W) writer structure
impl crate::Writable for FLASH_WAITI_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_WAITI_CTRL to value 0x14
impl crate::Resettable for FLASH_WAITI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
