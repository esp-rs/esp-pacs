#[doc = "Register `BLEADVCHMAP` reader"]
pub type R = crate::R<BLEADVCHMAP_SPEC>;
#[doc = "Register `BLEADVCHMAP` writer"]
pub type W = crate::W<BLEADVCHMAP_SPEC>;
#[doc = "Field `CHAN37` reader - Use channel 37 to send advertisements"]
pub type CHAN37_R = crate::BitReader;
#[doc = "Field `CHAN37` writer - Use channel 37 to send advertisements"]
pub type CHAN37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN38` reader - Use channel 38 to send advertisements"]
pub type CHAN38_R = crate::BitReader;
#[doc = "Field `CHAN38` writer - Use channel 38 to send advertisements"]
pub type CHAN38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN39` reader - Use channel 38 to send advertisements"]
pub type CHAN39_R = crate::BitReader;
#[doc = "Field `CHAN39` writer - Use channel 38 to send advertisements"]
pub type CHAN39_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Use channel 37 to send advertisements"]
    #[inline(always)]
    pub fn chan37(&self) -> CHAN37_R {
        CHAN37_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use channel 38 to send advertisements"]
    #[inline(always)]
    pub fn chan38(&self) -> CHAN38_R {
        CHAN38_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Use channel 38 to send advertisements"]
    #[inline(always)]
    pub fn chan39(&self) -> CHAN39_R {
        CHAN39_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEADVCHMAP")
            .field("chan37", &self.chan37())
            .field("chan38", &self.chan38())
            .field("chan39", &self.chan39())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Use channel 37 to send advertisements"]
    #[inline(always)]
    pub fn chan37(&mut self) -> CHAN37_W<'_, BLEADVCHMAP_SPEC> {
        CHAN37_W::new(self, 0)
    }
    #[doc = "Bit 1 - Use channel 38 to send advertisements"]
    #[inline(always)]
    pub fn chan38(&mut self) -> CHAN38_W<'_, BLEADVCHMAP_SPEC> {
        CHAN38_W::new(self, 1)
    }
    #[doc = "Bit 2 - Use channel 38 to send advertisements"]
    #[inline(always)]
    pub fn chan39(&mut self) -> CHAN39_W<'_, BLEADVCHMAP_SPEC> {
        CHAN39_W::new(self, 2)
    }
}
#[doc = "Advertising Channel Map\n\nYou can [`read`](crate::Reg::read) this register and get [`bleadvchmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleadvchmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEADVCHMAP_SPEC;
impl crate::RegisterSpec for BLEADVCHMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleadvchmap::R`](R) reader structure"]
impl crate::Readable for BLEADVCHMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleadvchmap::W`](W) writer structure"]
impl crate::Writable for BLEADVCHMAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEADVCHMAP to value 0"]
impl crate::Resettable for BLEADVCHMAP_SPEC {}
