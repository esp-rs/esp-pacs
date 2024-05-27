///Register `MPU_IA_INT_EN` reader
pub type R = crate::R<MPU_IA_INT_EN_SPEC>;
///Register `MPU_IA_INT_EN` writer
pub type W = crate::W<MPU_IA_INT_EN_SPEC>;
///Field `MPU_IA_INT_EN` reader -
pub type MPU_IA_INT_EN_R = crate::FieldReader<u32>;
///Field `MPU_IA_INT_EN` writer -
pub type MPU_IA_INT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bits 0:16
    #[inline(always)]
    pub fn mpu_ia_int_en(&self) -> MPU_IA_INT_EN_R {
        MPU_IA_INT_EN_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPU_IA_INT_EN")
            .field("mpu_ia_int_en", &self.mpu_ia_int_en())
            .finish()
    }
}
impl W {
    ///Bits 0:16
    #[inline(always)]
    #[must_use]
    pub fn mpu_ia_int_en(&mut self) -> MPU_IA_INT_EN_W<MPU_IA_INT_EN_SPEC> {
        MPU_IA_INT_EN_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_ia_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_ia_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MPU_IA_INT_EN_SPEC;
impl crate::RegisterSpec for MPU_IA_INT_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mpu_ia_int_en::R`](R) reader structure
impl crate::Readable for MPU_IA_INT_EN_SPEC {}
///`write(|w| ..)` method takes [`mpu_ia_int_en::W`](W) writer structure
impl crate::Writable for MPU_IA_INT_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MPU_IA_INT_EN to value 0
impl crate::Resettable for MPU_IA_INT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
