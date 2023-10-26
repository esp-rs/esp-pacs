#[doc = "Register `AHB_MPU_TABLE_1` reader"]
pub type R = crate::R<AHB_MPU_TABLE_1_SPEC>;
#[doc = "Register `AHB_MPU_TABLE_1` writer"]
pub type W = crate::W<AHB_MPU_TABLE_1_SPEC>;
#[doc = "Field `AHB_ACCESS_GRANT_1` reader - "]
pub type AHB_ACCESS_GRANT_1_R = crate::FieldReader<u16>;
#[doc = "Field `AHB_ACCESS_GRANT_1` writer - "]
pub type AHB_ACCESS_GRANT_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn ahb_access_grant_1(&self) -> AHB_ACCESS_GRANT_1_R {
        AHB_ACCESS_GRANT_1_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_MPU_TABLE_1")
            .field(
                "ahb_access_grant_1",
                &format_args!("{}", self.ahb_access_grant_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_MPU_TABLE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_access_grant_1(&mut self) -> AHB_ACCESS_GRANT_1_W<AHB_MPU_TABLE_1_SPEC, 0> {
        AHB_ACCESS_GRANT_1_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_mpu_table_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_mpu_table_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_MPU_TABLE_1_SPEC;
impl crate::RegisterSpec for AHB_MPU_TABLE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_mpu_table_1::R`](R) reader structure"]
impl crate::Readable for AHB_MPU_TABLE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_mpu_table_1::W`](W) writer structure"]
impl crate::Writable for AHB_MPU_TABLE_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_MPU_TABLE_1 to value 0x01ff"]
impl crate::Resettable for AHB_MPU_TABLE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
