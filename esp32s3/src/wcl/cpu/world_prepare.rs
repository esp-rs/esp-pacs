#[doc = "Register `WORLD_PREPARE` reader"]
pub type R = crate::R<WORLD_PREPARE_SPEC>;
#[doc = "Register `WORLD_PREPARE` writer"]
pub type W = crate::W<WORLD_PREPARE_SPEC>;
#[doc = "Field `WORLD_PREPARE` reader - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type WORLD_PREPARE_R = crate::FieldReader;
#[doc = "Field `WORLD_PREPARE` writer - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
pub type WORLD_PREPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    pub fn world_prepare(&self) -> WORLD_PREPARE_R {
        WORLD_PREPARE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORLD_PREPARE")
            .field("world_prepare", &self.world_prepare())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field to used to set world to enter, 2'b01 means WORLD0, 2'b10 means WORLD1"]
    #[inline(always)]
    pub fn world_prepare(&mut self) -> WORLD_PREPARE_W<WORLD_PREPARE_SPEC> {
        WORLD_PREPARE_W::new(self, 0)
    }
}
#[doc = "Core_0 prepare world configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_prepare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_prepare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORLD_PREPARE_SPEC;
impl crate::RegisterSpec for WORLD_PREPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`world_prepare::R`](R) reader structure"]
impl crate::Readable for WORLD_PREPARE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`world_prepare::W`](W) writer structure"]
impl crate::Writable for WORLD_PREPARE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORLD_PREPARE to value 0"]
impl crate::Resettable for WORLD_PREPARE_SPEC {}
