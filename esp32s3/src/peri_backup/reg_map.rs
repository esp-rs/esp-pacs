#[doc = "Register `REG_MAP%s` reader"]
pub type R = crate::R<REG_MAP_SPEC>;
#[doc = "Register `REG_MAP%s` writer"]
pub type W = crate::W<REG_MAP_SPEC>;
#[doc = "Field `MAP` reader - x"]
pub type MAP_R = crate::FieldReader<u32>;
#[doc = "Field `MAP` writer - x"]
pub type MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP").field("map", &self.map()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W<REG_MAP_SPEC> {
        MAP_W::new(self, 0)
    }
}
#[doc = "x\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP_SPEC;
impl crate::RegisterSpec for REG_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map::R`](R) reader structure"]
impl crate::Readable for REG_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map::W`](W) writer structure"]
impl crate::Writable for REG_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MAP%s to value 0"]
impl crate::Resettable for REG_MAP_SPEC {}
