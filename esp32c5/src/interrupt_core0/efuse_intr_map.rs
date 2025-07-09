#[doc = "Register `EFUSE_INTR_MAP` reader"]
pub type R = crate::R<EFUSE_INTR_MAP_SPEC>;
#[doc = "Register `EFUSE_INTR_MAP` writer"]
pub type W = crate::W<EFUSE_INTR_MAP_SPEC>;
#[doc = "Field `EFUSE_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type EFUSE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `EFUSE_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type EFUSE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn efuse_intr_map(&self) -> EFUSE_INTR_MAP_R {
        EFUSE_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE_INTR_MAP")
            .field("efuse_intr_map", &self.efuse_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn efuse_intr_map(&mut self) -> EFUSE_INTR_MAP_W<EFUSE_INTR_MAP_SPEC> {
        EFUSE_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "EFUSE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFUSE_INTR_MAP_SPEC;
impl crate::RegisterSpec for EFUSE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_intr_map::R`](R) reader structure"]
impl crate::Readable for EFUSE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`efuse_intr_map::W`](W) writer structure"]
impl crate::Writable for EFUSE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFUSE_INTR_MAP to value 0"]
impl crate::Resettable for EFUSE_INTR_MAP_SPEC {}
