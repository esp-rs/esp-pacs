#[doc = "Register `CORE_0_VECBASE_OVERRIDE_0` reader"]
pub type R = crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "Register `CORE_0_VECBASE_OVERRIDE_0` writer"]
pub type W = crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
#[doc = "Field `CORE_0_VECBASE_WORLD_MASK` reader - Set 1 to mask world, then only world0_value will work."]
pub type CORE_0_VECBASE_WORLD_MASK_R = crate::BitReader;
#[doc = "Field `CORE_0_VECBASE_WORLD_MASK` writer - Set 1 to mask world, then only world0_value will work."]
pub type CORE_0_VECBASE_WORLD_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask world, then only world0_value will work."]
    #[inline(always)]
    pub fn core_0_vecbase_world_mask(&self) -> CORE_0_VECBASE_WORLD_MASK_R {
        CORE_0_VECBASE_WORLD_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_VECBASE_OVERRIDE_0")
            .field(
                "core_0_vecbase_world_mask",
                &format_args!("{}", self.core_0_vecbase_world_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_VECBASE_OVERRIDE_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask world, then only world0_value will work."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_world_mask(
        &mut self,
    ) -> CORE_0_VECBASE_WORLD_MASK_W<CORE_0_VECBASE_OVERRIDE_0_SPEC, 0> {
        CORE_0_VECBASE_WORLD_MASK_W::new(self)
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
#[doc = "core0 vecbase override configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_VECBASE_OVERRIDE_0_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_vecbase_override_0::R`](R) reader structure"]
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_vecbase_override_0::W`](W) writer structure"]
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_VECBASE_OVERRIDE_0 to value 0x01"]
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
