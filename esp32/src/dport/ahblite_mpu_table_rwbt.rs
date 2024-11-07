#[doc = "Register `AHBLITE_MPU_TABLE_RWBT` reader"]
pub type R = crate::R<AHBLITE_MPU_TABLE_RWBT_SPEC>;
#[doc = "Register `AHBLITE_MPU_TABLE_RWBT` writer"]
pub type W = crate::W<AHBLITE_MPU_TABLE_RWBT_SPEC>;
#[doc = "Field `RWBT_ACCESS_GRANT_CONFIG` reader - "]
pub type RWBT_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
#[doc = "Field `RWBT_ACCESS_GRANT_CONFIG` writer - "]
pub type RWBT_ACCESS_GRANT_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rwbt_access_grant_config(&self) -> RWBT_ACCESS_GRANT_CONFIG_R {
        RWBT_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_RWBT")
            .field("rwbt_access_grant_config", &self.rwbt_access_grant_config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rwbt_access_grant_config(
        &mut self,
    ) -> RWBT_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_RWBT_SPEC> {
        RWBT_ACCESS_GRANT_CONFIG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblite_mpu_table_rwbt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblite_mpu_table_rwbt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLITE_MPU_TABLE_RWBT_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_RWBT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblite_mpu_table_rwbt::R`](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_RWBT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblite_mpu_table_rwbt::W`](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_RWBT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_RWBT to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_RWBT_SPEC {
    const RESET_VALUE: u32 = 0;
}
