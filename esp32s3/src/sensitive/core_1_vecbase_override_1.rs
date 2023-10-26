#[doc = "Register `CORE_1_VECBASE_OVERRIDE_1` reader"]
pub type R = crate::R<CORE_1_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "Register `CORE_1_VECBASE_OVERRIDE_1` writer"]
pub type W = crate::W<CORE_1_VECBASE_OVERRIDE_1_SPEC>;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE` reader - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
pub type CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE` writer - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
pub type CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 22, O, u32>;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_SEL` reader - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_1_VECBASE_OVERRIDE_SEL_R = crate::FieldReader;
#[doc = "Field `CORE_1_VECBASE_OVERRIDE_SEL` writer - Set 0x3 to sel vecbase_override to override vecbase register."]
pub type CORE_1_VECBASE_OVERRIDE_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_world0_value(&self) -> CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R {
        CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    pub fn core_1_vecbase_override_sel(&self) -> CORE_1_VECBASE_OVERRIDE_SEL_R {
        CORE_1_VECBASE_OVERRIDE_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_VECBASE_OVERRIDE_1")
            .field(
                "core_1_vecbase_override_world0_value",
                &format_args!("{}", self.core_1_vecbase_override_world0_value().bits()),
            )
            .field(
                "core_1_vecbase_override_sel",
                &format_args!("{}", self.core_1_vecbase_override_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_VECBASE_OVERRIDE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - world0 vecbase_override register, when core1 in world0 use this register to override vecbase register."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_vecbase_override_world0_value(
        &mut self,
    ) -> CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W<CORE_1_VECBASE_OVERRIDE_1_SPEC, 0> {
        CORE_1_VECBASE_OVERRIDE_WORLD0_VALUE_W::new(self)
    }
    #[doc = "Bits 22:23 - Set 0x3 to sel vecbase_override to override vecbase register."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_vecbase_override_sel(
        &mut self,
    ) -> CORE_1_VECBASE_OVERRIDE_SEL_W<CORE_1_VECBASE_OVERRIDE_1_SPEC, 22> {
        CORE_1_VECBASE_OVERRIDE_SEL_W::new(self)
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
#[doc = "core1 vecbase override configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_vecbase_override_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_vecbase_override_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_VECBASE_OVERRIDE_1_SPEC;
impl crate::RegisterSpec for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_vecbase_override_1::R`](R) reader structure"]
impl crate::Readable for CORE_1_VECBASE_OVERRIDE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_vecbase_override_1::W`](W) writer structure"]
impl crate::Writable for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_VECBASE_OVERRIDE_1 to value 0"]
impl crate::Resettable for CORE_1_VECBASE_OVERRIDE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
