#[doc = "Register `CORE_0_VECBASE_OVERRIDE_2` reader"]
pub type R = crate::R<CORE_0_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "Register `CORE_0_VECBASE_OVERRIDE_2` writer"]
pub type W = crate::W<CORE_0_VECBASE_OVERRIDE_2_SPEC>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE` reader - world1 vecbase_override register, when core0 in world1 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE` writer - world1 vecbase_override register, when core0 in world1 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - world1 vecbase_override register, when core0 in world1 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_world1_value(&self) -> CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_R {
        CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_VECBASE_OVERRIDE_2")
            .field(
                "core_0_vecbase_override_world1_value",
                &format_args!("{}", self.core_0_vecbase_override_world1_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_VECBASE_OVERRIDE_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - world1 vecbase_override register, when core0 in world1 use this register to override vecbase register."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_override_world1_value(
        &mut self,
    ) -> CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_W<CORE_0_VECBASE_OVERRIDE_2_SPEC> {
        CORE_0_VECBASE_OVERRIDE_WORLD1_VALUE_W::new(self, 0)
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
#[doc = "core0 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_VECBASE_OVERRIDE_2_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_vecbase_override_2::R`](R) reader structure"]
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_vecbase_override_2::W`](W) writer structure"]
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_VECBASE_OVERRIDE_2 to value 0"]
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
