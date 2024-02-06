#[doc = "Register `CORE_1_SP_MIN` reader"]
pub type R = crate::R<CORE_1_SP_MIN_SPEC>;
#[doc = "Register `CORE_1_SP_MIN` writer"]
pub type W = crate::W<CORE_1_SP_MIN_SPEC>;
#[doc = "Field `CORE_1_SP_MIN` reader - core1 sp region configuration regsiter"]
pub type CORE_1_SP_MIN_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_SP_MIN` writer - core1 sp region configuration regsiter"]
pub type CORE_1_SP_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core1 sp region configuration regsiter"]
    #[inline(always)]
    pub fn core_1_sp_min(&self) -> CORE_1_SP_MIN_R {
        CORE_1_SP_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_SP_MIN")
            .field(
                "core_1_sp_min",
                &format_args!("{}", self.core_1_sp_min().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_SP_MIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - core1 sp region configuration regsiter"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_min(&mut self) -> CORE_1_SP_MIN_W<CORE_1_SP_MIN_SPEC> {
        CORE_1_SP_MIN_W::new(self, 0)
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
#[doc = "stack min value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_min::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_sp_min::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_SP_MIN_SPEC;
impl crate::RegisterSpec for CORE_1_SP_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_sp_min::R`](R) reader structure"]
impl crate::Readable for CORE_1_SP_MIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_sp_min::W`](W) writer structure"]
impl crate::Writable for CORE_1_SP_MIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_SP_MIN to value 0"]
impl crate::Resettable for CORE_1_SP_MIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
