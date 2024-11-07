#[doc = "Register `PGM_DATA%s` reader"]
pub type R = crate::R<PGM_DATA_SPEC>;
#[doc = "Register `PGM_DATA%s` writer"]
pub type W = crate::W<PGM_DATA_SPEC>;
#[doc = "Field `PGM_DATA` reader - The content of the %sth 32-bit data to be programmed."]
pub type PGM_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `PGM_DATA` writer - The content of the %sth 32-bit data to be programmed."]
pub type PGM_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data(&self) -> PGM_DATA_R {
        PGM_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_DATA")
            .field("pgm_data", &self.pgm_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data(&mut self) -> PGM_DATA_W<PGM_DATA_SPEC> {
        PGM_DATA_W::new(self, 0)
    }
}
#[doc = "Register %s that stores data to be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PGM_DATA_SPEC;
impl crate::RegisterSpec for PGM_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_data::R`](R) reader structure"]
impl crate::Readable for PGM_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pgm_data::W`](W) writer structure"]
impl crate::Writable for PGM_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGM_DATA%s to value 0"]
impl crate::Resettable for PGM_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
