#[doc = "Register `WORLD_UPDATE` writer"]
pub type W = crate::W<WORLD_UPDATE_SPEC>;
#[doc = "Field `UPDATE` writer - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
pub type UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WORLD_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to update configuration completed, can write any value,the hardware only checks the write operation of this register and does not case about its value"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<WORLD_UPDATE_SPEC> {
        UPDATE_W::new(self, 0)
    }
}
#[doc = "Core_0 configuration update register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORLD_UPDATE_SPEC;
impl crate::RegisterSpec for WORLD_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`world_update::W`](W) writer structure"]
impl crate::Writable for WORLD_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORLD_UPDATE to value 0"]
impl crate::Resettable for WORLD_UPDATE_SPEC {}
