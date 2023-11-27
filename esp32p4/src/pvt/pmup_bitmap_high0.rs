#[doc = "Register `PMUP_BITMAP_HIGH0` reader"]
pub type R = crate::R<PMUP_BITMAP_HIGH0_SPEC>;
#[doc = "Register `PMUP_BITMAP_HIGH0` writer"]
pub type W = crate::W<PMUP_BITMAP_HIGH0_SPEC>;
#[doc = "Field `PUMP_BITMAP_HIGH0` reader - select valid high channel0"]
pub type PUMP_BITMAP_HIGH0_R = crate::FieldReader<u32>;
#[doc = "Field `PUMP_BITMAP_HIGH0` writer - select valid high channel0"]
pub type PUMP_BITMAP_HIGH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - select valid high channel0"]
    #[inline(always)]
    pub fn pump_bitmap_high0(&self) -> PUMP_BITMAP_HIGH0_R {
        PUMP_BITMAP_HIGH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_BITMAP_HIGH0")
            .field(
                "pump_bitmap_high0",
                &format_args!("{}", self.pump_bitmap_high0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMUP_BITMAP_HIGH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - select valid high channel0"]
    #[inline(always)]
    #[must_use]
    pub fn pump_bitmap_high0(&mut self) -> PUMP_BITMAP_HIGH0_W<PMUP_BITMAP_HIGH0_SPEC> {
        PUMP_BITMAP_HIGH0_W::new(self, 0)
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
#[doc = "select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_BITMAP_HIGH0_SPEC;
impl crate::RegisterSpec for PMUP_BITMAP_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_bitmap_high0::R`](R) reader structure"]
impl crate::Readable for PMUP_BITMAP_HIGH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_bitmap_high0::W`](W) writer structure"]
impl crate::Writable for PMUP_BITMAP_HIGH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMUP_BITMAP_HIGH0 to value 0"]
impl crate::Resettable for PMUP_BITMAP_HIGH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
