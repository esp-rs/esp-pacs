#[doc = "Register `PCNT_INTR_MAP` reader"]
pub type R = crate::R<PCNT_INTR_MAP_SPEC>;
#[doc = "Register `PCNT_INTR_MAP` writer"]
pub type W = crate::W<PCNT_INTR_MAP_SPEC>;
#[doc = "Field `PCNT_INTR_MAP` reader - CORE0_PCNT_INTR mapping register"]
pub type PCNT_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PCNT_INTR_MAP` writer - CORE0_PCNT_INTR mapping register"]
pub type PCNT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_PCNT_INTR mapping register"]
    #[inline(always)]
    pub fn pcnt_intr_map(&self) -> PCNT_INTR_MAP_R {
        PCNT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT_INTR_MAP")
            .field("pcnt_intr_map", &self.pcnt_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_PCNT_INTR mapping register"]
    #[inline(always)]
    pub fn pcnt_intr_map(&mut self) -> PCNT_INTR_MAP_W<PCNT_INTR_MAP_SPEC> {
        PCNT_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNT_INTR_MAP_SPEC;
impl crate::RegisterSpec for PCNT_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt_intr_map::R`](R) reader structure"]
impl crate::Readable for PCNT_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcnt_intr_map::W`](W) writer structure"]
impl crate::Writable for PCNT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNT_INTR_MAP to value 0"]
impl crate::Resettable for PCNT_INTR_MAP_SPEC {}
