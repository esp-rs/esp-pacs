#[doc = "Register `AHB_MPU_TABLE_0` reader"]
pub type R = crate::R<AHB_MPU_TABLE_0_SPEC>;
#[doc = "Register `AHB_MPU_TABLE_0` writer"]
pub type W = crate::W<AHB_MPU_TABLE_0_SPEC>;
#[doc = "Field `AHB_ACCESS_GRANT_0` reader - "]
pub type AHB_ACCESS_GRANT_0_R = crate::FieldReader<u32>;
#[doc = "Field `AHB_ACCESS_GRANT_0` writer - "]
pub type AHB_ACCESS_GRANT_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ahb_access_grant_0(&self) -> AHB_ACCESS_GRANT_0_R {
        AHB_ACCESS_GRANT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_MPU_TABLE_0")
            .field(
                "ahb_access_grant_0",
                &format_args!("{}", self.ahb_access_grant_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_MPU_TABLE_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_access_grant_0(&mut self) -> AHB_ACCESS_GRANT_0_W<AHB_MPU_TABLE_0_SPEC, 0> {
        AHB_ACCESS_GRANT_0_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_mpu_table_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_mpu_table_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_MPU_TABLE_0_SPEC;
impl crate::RegisterSpec for AHB_MPU_TABLE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_mpu_table_0::R`](R) reader structure"]
impl crate::Readable for AHB_MPU_TABLE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_mpu_table_0::W`](W) writer structure"]
impl crate::Writable for AHB_MPU_TABLE_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_MPU_TABLE_0 to value 0xffff_ffff"]
impl crate::Resettable for AHB_MPU_TABLE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
