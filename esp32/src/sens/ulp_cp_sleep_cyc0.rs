#[doc = "Register `ULP_CP_SLEEP_CYC0` reader"]
pub type R = crate::R<ULP_CP_SLEEP_CYC0_SPEC>;
#[doc = "Register `ULP_CP_SLEEP_CYC0` writer"]
pub type W = crate::W<ULP_CP_SLEEP_CYC0_SPEC>;
#[doc = "Field `SLEEP_CYCLES_S0` reader - sleep cycles for ULP-coprocessor timer"]
pub type SLEEP_CYCLES_S0_R = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_CYCLES_S0` writer - sleep cycles for ULP-coprocessor timer"]
pub type SLEEP_CYCLES_S0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sleep_cycles_s0(&self) -> SLEEP_CYCLES_S0_R {
        SLEEP_CYCLES_S0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC0")
            .field(
                "sleep_cycles_s0",
                &format_args!("{}", self.sleep_cycles_s0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_SLEEP_CYC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_cycles_s0(&mut self) -> SLEEP_CYCLES_S0_W<ULP_CP_SLEEP_CYC0_SPEC, 0> {
        SLEEP_CYCLES_S0_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_sleep_cyc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULP_CP_SLEEP_CYC0_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulp_cp_sleep_cyc0::R`](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulp_cp_sleep_cyc0::W`](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC0 to value 0xc8"]
impl crate::Resettable for ULP_CP_SLEEP_CYC0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc8;
}
