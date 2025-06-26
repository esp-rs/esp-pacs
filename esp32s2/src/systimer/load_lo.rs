#[doc = "Register `LOAD_LO` reader"]
pub type R = crate::R<LOAD_LO_SPEC>;
#[doc = "Register `LOAD_LO` writer"]
pub type W = crate::W<LOAD_LO_SPEC>;
#[doc = "Field `LOAD_LO` reader - The value to be loaded into system timer, low 32 bits."]
pub type LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - The value to be loaded into system timer, low 32 bits."]
pub type LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, low 32 bits."]
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD_LO")
            .field("load_lo", &self.load_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, low 32 bits."]
    #[inline(always)]
    pub fn load_lo(&mut self) -> LOAD_LO_W<LOAD_LO_SPEC> {
        LOAD_LO_W::new(self, 0)
    }
}
#[doc = "Low 32 bits to be loaded to system timer\n\nYou can [`read`](crate::Reg::read) this register and get [`load_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_LO_SPEC;
impl crate::RegisterSpec for LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_lo::R`](R) reader structure"]
impl crate::Readable for LOAD_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_lo::W`](W) writer structure"]
impl crate::Writable for LOAD_LO_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LOAD_LO to value 0"]
impl crate::Resettable for LOAD_LO_SPEC {}
