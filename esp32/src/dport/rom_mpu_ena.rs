///Register `ROM_MPU_ENA` reader
pub type R = crate::R<ROM_MPU_ENA_SPEC>;
///Register `ROM_MPU_ENA` writer
pub type W = crate::W<ROM_MPU_ENA_SPEC>;
///Field `SHARE_ROM_MPU_ENA` reader -
pub type SHARE_ROM_MPU_ENA_R = crate::BitReader;
///Field `SHARE_ROM_MPU_ENA` writer -
pub type SHARE_ROM_MPU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ROM_MPU_ENA` reader -
pub type PRO_ROM_MPU_ENA_R = crate::BitReader;
///Field `PRO_ROM_MPU_ENA` writer -
pub type PRO_ROM_MPU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_ROM_MPU_ENA` reader -
pub type APP_ROM_MPU_ENA_R = crate::BitReader;
///Field `APP_ROM_MPU_ENA` writer -
pub type APP_ROM_MPU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn share_rom_mpu_ena(&self) -> SHARE_ROM_MPU_ENA_R {
        SHARE_ROM_MPU_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn pro_rom_mpu_ena(&self) -> PRO_ROM_MPU_ENA_R {
        PRO_ROM_MPU_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn app_rom_mpu_ena(&self) -> APP_ROM_MPU_ENA_R {
        APP_ROM_MPU_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_MPU_ENA")
            .field("share_rom_mpu_ena", &self.share_rom_mpu_ena())
            .field("pro_rom_mpu_ena", &self.pro_rom_mpu_ena())
            .field("app_rom_mpu_ena", &self.app_rom_mpu_ena())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn share_rom_mpu_ena(&mut self) -> SHARE_ROM_MPU_ENA_W<ROM_MPU_ENA_SPEC> {
        SHARE_ROM_MPU_ENA_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn pro_rom_mpu_ena(&mut self) -> PRO_ROM_MPU_ENA_W<ROM_MPU_ENA_SPEC> {
        PRO_ROM_MPU_ENA_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn app_rom_mpu_ena(&mut self) -> APP_ROM_MPU_ENA_W<ROM_MPU_ENA_SPEC> {
        APP_ROM_MPU_ENA_W::new(self, 2)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ROM_MPU_ENA_SPEC;
impl crate::RegisterSpec for ROM_MPU_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rom_mpu_ena::R`](R) reader structure
impl crate::Readable for ROM_MPU_ENA_SPEC {}
///`write(|w| ..)` method takes [`rom_mpu_ena::W`](W) writer structure
impl crate::Writable for ROM_MPU_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROM_MPU_ENA to value 0
impl crate::Resettable for ROM_MPU_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
