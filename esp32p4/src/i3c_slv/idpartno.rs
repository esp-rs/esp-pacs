#[doc = "Register `IDPARTNO` reader"]
pub type R = crate::R<IDPARTNO_SPEC>;
#[doc = "Register `IDPARTNO` writer"]
pub type W = crate::W<IDPARTNO_SPEC>;
#[doc = "Field `PARTNO` reader - NA"]
pub type PARTNO_R = crate::FieldReader<u32>;
#[doc = "Field `PARTNO` writer - NA"]
pub type PARTNO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDPARTNO")
            .field("partno", &self.partno())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn partno(&mut self) -> PARTNO_W<'_, IDPARTNO_SPEC> {
        PARTNO_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`idpartno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idpartno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDPARTNO_SPEC;
impl crate::RegisterSpec for IDPARTNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idpartno::R`](R) reader structure"]
impl crate::Readable for IDPARTNO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idpartno::W`](W) writer structure"]
impl crate::Writable for IDPARTNO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDPARTNO to value 0"]
impl crate::Resettable for IDPARTNO_SPEC {}
