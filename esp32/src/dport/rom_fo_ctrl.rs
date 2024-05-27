///Register `ROM_FO_CTRL` reader
pub type R = crate::R<ROM_FO_CTRL_SPEC>;
///Register `ROM_FO_CTRL` writer
pub type W = crate::W<ROM_FO_CTRL_SPEC>;
///Field `PRO_ROM_FO` reader -
pub type PRO_ROM_FO_R = crate::BitReader;
///Field `PRO_ROM_FO` writer -
pub type PRO_ROM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_ROM_FO` reader -
pub type APP_ROM_FO_R = crate::BitReader;
///Field `APP_ROM_FO` writer -
pub type APP_ROM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHARE_ROM_FO` reader -
pub type SHARE_ROM_FO_R = crate::FieldReader;
///Field `SHARE_ROM_FO` writer -
pub type SHARE_ROM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn pro_rom_fo(&self) -> PRO_ROM_FO_R {
        PRO_ROM_FO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn app_rom_fo(&self) -> APP_ROM_FO_R {
        APP_ROM_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:7
    #[inline(always)]
    pub fn share_rom_fo(&self) -> SHARE_ROM_FO_R {
        SHARE_ROM_FO_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_FO_CTRL")
            .field("pro_rom_fo", &self.pro_rom_fo())
            .field("app_rom_fo", &self.app_rom_fo())
            .field("share_rom_fo", &self.share_rom_fo())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn pro_rom_fo(&mut self) -> PRO_ROM_FO_W<ROM_FO_CTRL_SPEC> {
        PRO_ROM_FO_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn app_rom_fo(&mut self) -> APP_ROM_FO_W<ROM_FO_CTRL_SPEC> {
        APP_ROM_FO_W::new(self, 1)
    }
    ///Bits 2:7
    #[inline(always)]
    #[must_use]
    pub fn share_rom_fo(&mut self) -> SHARE_ROM_FO_W<ROM_FO_CTRL_SPEC> {
        SHARE_ROM_FO_W::new(self, 2)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rom_fo_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_fo_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ROM_FO_CTRL_SPEC;
impl crate::RegisterSpec for ROM_FO_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rom_fo_ctrl::R`](R) reader structure
impl crate::Readable for ROM_FO_CTRL_SPEC {}
///`write(|w| ..)` method takes [`rom_fo_ctrl::W`](W) writer structure
impl crate::Writable for ROM_FO_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROM_FO_CTRL to value 0x03
impl crate::Resettable for ROM_FO_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
