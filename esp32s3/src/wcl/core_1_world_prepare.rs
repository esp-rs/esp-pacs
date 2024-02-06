#[doc = "Register `Core_1_World_PREPARE` reader"]
pub type R = crate::R<CORE_1_WORLD_PREPARE_SPEC>;
#[doc = "Register `Core_1_World_PREPARE` writer"]
pub type W = crate::W<CORE_1_WORLD_PREPARE_SPEC>;
#[doc = "Field `CORE_1_WORLD_PREPARE` reader - This field to used to set world to enter,2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type CORE_1_WORLD_PREPARE_R = crate::FieldReader;
#[doc = "Field `CORE_1_WORLD_PREPARE` writer - This field to used to set world to enter,2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type CORE_1_WORLD_PREPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This field to used to set world to enter,2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    pub fn core_1_world_prepare(&self) -> CORE_1_WORLD_PREPARE_R {
        CORE_1_WORLD_PREPARE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_World_PREPARE")
            .field(
                "core_1_world_prepare",
                &format_args!("{}", self.core_1_world_prepare().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_WORLD_PREPARE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field to used to set world to enter,2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_world_prepare(&mut self) -> CORE_1_WORLD_PREPARE_W<CORE_1_WORLD_PREPARE_SPEC> {
        CORE_1_WORLD_PREPARE_W::new(self, 0)
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
#[doc = "Core_1 prepare world configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_world_prepare::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_world_prepare::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_WORLD_PREPARE_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_PREPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_world_prepare::R`](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_PREPARE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_world_prepare::W`](W) writer structure"]
impl crate::Writable for CORE_1_WORLD_PREPARE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_1_World_PREPARE to value 0"]
impl crate::Resettable for CORE_1_WORLD_PREPARE_SPEC {
    const RESET_VALUE: u32 = 0;
}
