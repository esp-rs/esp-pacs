#[doc = "Register `Core_0_World_IRam0` reader"]
pub type R = crate::R<CORE_0_WORLD_IRAM0_SPEC>;
#[doc = "Register `Core_0_World_IRam0` writer"]
pub type W = crate::W<CORE_0_WORLD_IRAM0_SPEC>;
#[doc = "Field `CORE_0_WORLD_IRAM0` reader - this field is used to read current world of Iram0 bus"]
pub type CORE_0_WORLD_IRAM0_R = crate::FieldReader;
#[doc = "Field `CORE_0_WORLD_IRAM0` writer - this field is used to read current world of Iram0 bus"]
pub type CORE_0_WORLD_IRAM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_0_world_iram0(&self) -> CORE_0_WORLD_IRAM0_R {
        CORE_0_WORLD_IRAM0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_World_IRam0")
            .field("core_0_world_iram0", &self.core_0_world_iram0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Iram0 bus"]
    #[inline(always)]
    pub fn core_0_world_iram0(&mut self) -> CORE_0_WORLD_IRAM0_W<CORE_0_WORLD_IRAM0_SPEC> {
        CORE_0_WORLD_IRAM0_W::new(self, 0)
    }
}
#[doc = "Core_0 Iram0 world register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_world_iram0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_world_iram0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_WORLD_IRAM0_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_IRAM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_world_iram0::R`](R) reader structure"]
impl crate::Readable for CORE_0_WORLD_IRAM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_world_iram0::W`](W) writer structure"]
impl crate::Writable for CORE_0_WORLD_IRAM0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_0_World_IRam0 to value 0"]
impl crate::Resettable for CORE_0_WORLD_IRAM0_SPEC {}
