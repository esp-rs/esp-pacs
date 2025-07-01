#[doc = "Register `SCRAMBLING` reader"]
pub type R = crate::R<SCRAMBLING_SPEC>;
#[doc = "Register `SCRAMBLING` writer"]
pub type W = crate::W<SCRAMBLING_SPEC>;
#[doc = "Field `SCRAMBLE_ENABLE` reader - NA"]
pub type SCRAMBLE_ENABLE_R = crate::BitReader;
#[doc = "Field `SCRAMBLE_ENABLE` writer - NA"]
pub type SCRAMBLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn scramble_enable(&self) -> SCRAMBLE_ENABLE_R {
        SCRAMBLE_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCRAMBLING")
            .field("scramble_enable", &self.scramble_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn scramble_enable(&mut self) -> SCRAMBLE_ENABLE_W<SCRAMBLING_SPEC> {
        SCRAMBLE_ENABLE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBLING_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambling::R`](R) reader structure"]
impl crate::Readable for SCRAMBLING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scrambling::W`](W) writer structure"]
impl crate::Writable for SCRAMBLING_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCRAMBLING to value 0"]
impl crate::Resettable for SCRAMBLING_SPEC {}
