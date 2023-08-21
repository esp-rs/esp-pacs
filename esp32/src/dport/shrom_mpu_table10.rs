#[doc = "Register `SHROM_MPU_TABLE10` reader"]
pub type R = crate::R<SHROM_MPU_TABLE10_SPEC>;
#[doc = "Register `SHROM_MPU_TABLE10` writer"]
pub type W = crate::W<SHROM_MPU_TABLE10_SPEC>;
#[doc = "Field `SHROM_MPU_TABLE10` reader - "]
pub type SHROM_MPU_TABLE10_R = crate::FieldReader;
#[doc = "Field `SHROM_MPU_TABLE10` writer - "]
pub type SHROM_MPU_TABLE10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table10(&self) -> SHROM_MPU_TABLE10_R {
        SHROM_MPU_TABLE10_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHROM_MPU_TABLE10")
            .field(
                "shrom_mpu_table10",
                &format_args!("{}", self.shrom_mpu_table10().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHROM_MPU_TABLE10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn shrom_mpu_table10(&mut self) -> SHROM_MPU_TABLE10_W<SHROM_MPU_TABLE10_SPEC, 0> {
        SHROM_MPU_TABLE10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHROM_MPU_TABLE10_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shrom_mpu_table10::R`](R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shrom_mpu_table10::W`](W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHROM_MPU_TABLE10 to value 0x01"]
impl crate::Resettable for SHROM_MPU_TABLE10_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
