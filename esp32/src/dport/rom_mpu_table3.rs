///Register `ROM_MPU_TABLE3` reader
pub type R = crate::R<ROM_MPU_TABLE3_SPEC>;
///Register `ROM_MPU_TABLE3` writer
pub type W = crate::W<ROM_MPU_TABLE3_SPEC>;
///Field `ROM_MPU_TABLE3` reader -
pub type ROM_MPU_TABLE3_R = crate::FieldReader;
///Field `ROM_MPU_TABLE3` writer -
pub type ROM_MPU_TABLE3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn rom_mpu_table3(&self) -> ROM_MPU_TABLE3_R {
        ROM_MPU_TABLE3_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_MPU_TABLE3")
            .field("rom_mpu_table3", &self.rom_mpu_table3())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn rom_mpu_table3(&mut self) -> ROM_MPU_TABLE3_W<ROM_MPU_TABLE3_SPEC> {
        ROM_MPU_TABLE3_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rom_mpu_table3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_mpu_table3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ROM_MPU_TABLE3_SPEC;
impl crate::RegisterSpec for ROM_MPU_TABLE3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rom_mpu_table3::R`](R) reader structure
impl crate::Readable for ROM_MPU_TABLE3_SPEC {}
///`write(|w| ..)` method takes [`rom_mpu_table3::W`](W) writer structure
impl crate::Writable for ROM_MPU_TABLE3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROM_MPU_TABLE3 to value 0x01
impl crate::Resettable for ROM_MPU_TABLE3_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
