#[doc = "Register `AHBLITE_MPU_TABLE_MISC` reader"]
pub type R = crate::R<AHBLITE_MPU_TABLE_MISC_SPEC>;
#[doc = "Register `AHBLITE_MPU_TABLE_MISC` writer"]
pub type W = crate::W<AHBLITE_MPU_TABLE_MISC_SPEC>;
#[doc = "Field `MISC_ACCESS_GRANT_CONFIG` reader - "]
pub type MISC_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
#[doc = "Field `MISC_ACCESS_GRANT_CONFIG` writer - "]
pub type MISC_ACCESS_GRANT_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn misc_access_grant_config(&self) -> MISC_ACCESS_GRANT_CONFIG_R {
        MISC_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_MISC")
            .field(
                "misc_access_grant_config",
                &format_args!("{}", self.misc_access_grant_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHBLITE_MPU_TABLE_MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn misc_access_grant_config(
        &mut self,
    ) -> MISC_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_MISC_SPEC> {
        MISC_ACCESS_GRANT_CONFIG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLITE_MPU_TABLE_MISC_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblite_mpu_table_misc::R`](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblite_mpu_table_misc::W`](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_MISC to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
