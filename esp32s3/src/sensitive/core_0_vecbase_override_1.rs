#[doc = "Register `CORE_0_VECBASE_OVERRIDE_1` reader"]
pub type R = crate::R<CORE_0_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "Register `CORE_0_VECBASE_OVERRIDE_1` writer"]
pub type W = crate::W<CORE_0_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE` reader - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE` writer - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_SEL` reader - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_SEL_R = crate::FieldReader;
#[doc = "Field `CORE_0_VECBASE_OVERRIDE_SEL` writer - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_0_VECBASE_OVERRIDE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_world0_value(&self) -> CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R {
        CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_0_vecbase_override_sel(&self) -> CORE_0_VECBASE_OVERRIDE_SEL_R {
        CORE_0_VECBASE_OVERRIDE_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_VECBASE_OVERRIDE_1")
            .field(
                "core_0_vecbase_override_world0_value",
                &self.core_0_vecbase_override_world0_value(),
            )
            .field(
                "core_0_vecbase_override_sel",
                &self.core_0_vecbase_override_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core0 in world0 use this register to override vecbase register."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_override_world0_value(
        &mut self,
    ) -> CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W<CORE_0_VECBASE_OVERRIDE_1_SPEC> {
        CORE_0_VECBASE_OVERRIDE_WORLD0_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_override_sel(
        &mut self,
    ) -> CORE_0_VECBASE_OVERRIDE_SEL_W<CORE_0_VECBASE_OVERRIDE_1_SPEC> {
        CORE_0_VECBASE_OVERRIDE_SEL_W::new(self, 22)
    }
}
#[doc = "core0 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_VECBASE_OVERRIDE_1_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_vecbase_override_1::R`](R) reader structure"]
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_vecbase_override_1::W`](W) writer structure"]
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_VECBASE_OVERRIDE_1 to value 0"]
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
