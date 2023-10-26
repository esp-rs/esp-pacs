#[doc = "Register `AHBLITE_MPU_TABLE_EMAC` reader"]
pub type R = crate::R<AHBLITE_MPU_TABLE_EMAC_SPEC>;
#[doc = "Register `AHBLITE_MPU_TABLE_EMAC` writer"]
pub type W = crate::W<AHBLITE_MPU_TABLE_EMAC_SPEC>;
#[doc = "Field `EMAC_ACCESS_GRANT_CONFIG` reader - "]
pub type EMAC_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
#[doc = "Field `EMAC_ACCESS_GRANT_CONFIG` writer - "]
pub type EMAC_ACCESS_GRANT_CONFIG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn emac_access_grant_config(&self) -> EMAC_ACCESS_GRANT_CONFIG_R {
        EMAC_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_EMAC")
            .field(
                "emac_access_grant_config",
                &format_args!("{}", self.emac_access_grant_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHBLITE_MPU_TABLE_EMAC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn emac_access_grant_config(
        &mut self,
    ) -> EMAC_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_EMAC_SPEC, 0> {
        EMAC_ACCESS_GRANT_CONFIG_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_emac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_emac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLITE_MPU_TABLE_EMAC_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_EMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblite_mpu_table_emac::R`](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_EMAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblite_mpu_table_emac::W`](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_EMAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_EMAC to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_EMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
