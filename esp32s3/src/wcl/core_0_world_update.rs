#[doc = "Register `Core_0_World_UPDATE` writer"]
pub type W = crate::W<CORE_0_WORLD_UPDATE_SPEC>;
#[doc = "Field `CORE_0_UPDATE` writer - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
pub type CORE_0_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_WORLD_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_update(&mut self) -> CORE_0_UPDATE_W<CORE_0_WORLD_UPDATE_SPEC> {
        CORE_0_UPDATE_W::new(self, 0)
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
#[doc = "Core_0 configuration update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_world_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_WORLD_UPDATE_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_0_world_update::W`](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_0_World_UPDATE to value 0"]
impl crate::Resettable for CORE_0_WORLD_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
