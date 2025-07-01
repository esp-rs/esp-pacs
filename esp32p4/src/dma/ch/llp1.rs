#[doc = "Register `LLP1` reader"]
pub type R = crate::R<LLP1_SPEC>;
#[doc = "Register `LLP1` writer"]
pub type W = crate::W<LLP1_SPEC>;
#[doc = "Field `CH1_LOC1` reader - NA"]
pub type CH1_LOC1_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_LOC1` writer - NA"]
pub type CH1_LOC1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc1(&self) -> CH1_LOC1_R {
        CH1_LOC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLP1")
            .field("ch1_loc1", &self.ch1_loc1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc1(&mut self) -> CH1_LOC1_W<LLP1_SPEC> {
        CH1_LOC1_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`llp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLP1_SPEC;
impl crate::RegisterSpec for LLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp1::R`](R) reader structure"]
impl crate::Readable for LLP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llp1::W`](W) writer structure"]
impl crate::Writable for LLP1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP1 to value 0"]
impl crate::Resettable for LLP1_SPEC {}
