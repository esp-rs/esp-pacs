#[doc = "Register `SOSC_DFREQ` reader"]
pub type R = crate::R<SOSC_DFREQ_SPEC>;
#[doc = "Register `SOSC_DFREQ` writer"]
pub type W = crate::W<SOSC_DFREQ_SPEC>;
#[doc = "Field `SOSC_DFREQ` reader - need_des"]
pub type SOSC_DFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `SOSC_DFREQ` writer - need_des"]
pub type SOSC_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn sosc_dfreq(&self) -> SOSC_DFREQ_R {
        SOSC_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOSC_DFREQ")
            .field("sosc_dfreq", &self.sosc_dfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn sosc_dfreq(&mut self) -> SOSC_DFREQ_W<'_, SOSC_DFREQ_SPEC> {
        SOSC_DFREQ_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sosc_dfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosc_dfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOSC_DFREQ_SPEC;
impl crate::RegisterSpec for SOSC_DFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sosc_dfreq::R`](R) reader structure"]
impl crate::Readable for SOSC_DFREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sosc_dfreq::W`](W) writer structure"]
impl crate::Writable for SOSC_DFREQ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOSC_DFREQ to value 0x2b00_0000"]
impl crate::Resettable for SOSC_DFREQ_SPEC {
    const RESET_VALUE: u32 = 0x2b00_0000;
}
