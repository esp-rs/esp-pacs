#[doc = "Register `WORLD_TRIGGER_ADDR` reader"]
pub type R = crate::R<WORLD_TRIGGER_ADDR_SPEC>;
#[doc = "Register `WORLD_TRIGGER_ADDR` writer"]
pub type W = crate::W<WORLD_TRIGGER_ADDR_SPEC>;
#[doc = "Field `WORLD_TRIGGER_ADDR` reader - This field is used to configure the entry address from WORLD0 to WORLD1,when the CPU executes to this address,switch to WORLD1"]
pub type WORLD_TRIGGER_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `WORLD_TRIGGER_ADDR` writer - This field is used to configure the entry address from WORLD0 to WORLD1,when the CPU executes to this address,switch to WORLD1"]
pub type WORLD_TRIGGER_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field is used to configure the entry address from WORLD0 to WORLD1,when the CPU executes to this address,switch to WORLD1"]
    #[inline(always)]
    pub fn world_trigger_addr(&self) -> WORLD_TRIGGER_ADDR_R {
        WORLD_TRIGGER_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORLD_TRIGGER_ADDR")
            .field("world_trigger_addr", &self.world_trigger_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to configure the entry address from WORLD0 to WORLD1,when the CPU executes to this address,switch to WORLD1"]
    #[inline(always)]
    pub fn world_trigger_addr(&mut self) -> WORLD_TRIGGER_ADDR_W<WORLD_TRIGGER_ADDR_SPEC> {
        WORLD_TRIGGER_ADDR_W::new(self, 0)
    }
}
#[doc = "Core_0 trigger address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_trigger_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_trigger_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORLD_TRIGGER_ADDR_SPEC;
impl crate::RegisterSpec for WORLD_TRIGGER_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`world_trigger_addr::R`](R) reader structure"]
impl crate::Readable for WORLD_TRIGGER_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`world_trigger_addr::W`](W) writer structure"]
impl crate::Writable for WORLD_TRIGGER_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORLD_TRIGGER_ADDR to value 0"]
impl crate::Resettable for WORLD_TRIGGER_ADDR_SPEC {}
