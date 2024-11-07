#[doc = "Register `ULP_CP_SLEEP_CYC4` reader"]
pub type R = crate::R<ULP_CP_SLEEP_CYC4_SPEC>;
#[doc = "Register `ULP_CP_SLEEP_CYC4` writer"]
pub type W = crate::W<ULP_CP_SLEEP_CYC4_SPEC>;
#[doc = "Field `SLEEP_CYCLES_S4` reader - "]
pub type SLEEP_CYCLES_S4_R = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_CYCLES_S4` writer - "]
pub type SLEEP_CYCLES_S4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s4(&self) -> SLEEP_CYCLES_S4_R {
        SLEEP_CYCLES_S4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC4")
            .field("sleep_cycles_s4", &self.sleep_cycles_s4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s4(&mut self) -> SLEEP_CYCLES_S4_W<ULP_CP_SLEEP_CYC4_SPEC> {
        SLEEP_CYCLES_S4_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_sleep_cyc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULP_CP_SLEEP_CYC4_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulp_cp_sleep_cyc4::R`](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulp_cp_sleep_cyc4::W`](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC4 to value 0x14"]
impl crate::Resettable for ULP_CP_SLEEP_CYC4_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
