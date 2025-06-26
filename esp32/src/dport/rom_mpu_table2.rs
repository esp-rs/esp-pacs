#[doc = "Register `ROM_MPU_TABLE2` reader"]
pub type R = crate::R<ROM_MPU_TABLE2_SPEC>;
#[doc = "Register `ROM_MPU_TABLE2` writer"]
pub type W = crate::W<ROM_MPU_TABLE2_SPEC>;
#[doc = "Field `ROM_MPU_TABLE2` reader - "]
pub type ROM_MPU_TABLE2_R = crate::FieldReader;
#[doc = "Field `ROM_MPU_TABLE2` writer - "]
pub type ROM_MPU_TABLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table2(&self) -> ROM_MPU_TABLE2_R {
        ROM_MPU_TABLE2_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_MPU_TABLE2")
            .field("rom_mpu_table2", &self.rom_mpu_table2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table2(&mut self) -> ROM_MPU_TABLE2_W<ROM_MPU_TABLE2_SPEC> {
        ROM_MPU_TABLE2_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_mpu_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_mpu_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_MPU_TABLE2_SPEC;
impl crate::RegisterSpec for ROM_MPU_TABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_mpu_table2::R`](R) reader structure"]
impl crate::Readable for ROM_MPU_TABLE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_mpu_table2::W`](W) writer structure"]
impl crate::Writable for ROM_MPU_TABLE2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_MPU_TABLE2 to value 0x01"]
impl crate::Resettable for ROM_MPU_TABLE2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
