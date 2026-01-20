#[doc = "Register `BLEADVTIM` reader"]
pub type R = crate::R<BLEADVTIM_SPEC>;
#[doc = "Register `BLEADVTIM` writer"]
pub type W = crate::W<BLEADVTIM_SPEC>;
#[doc = "Field `ADVINT` reader - Time in us between sent advertisement packets"]
pub type ADVINT_R = crate::FieldReader<u16>;
#[doc = "Field `ADVINT` writer - Time in us between sent advertisement packets"]
pub type ADVINT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Time in us between sent advertisement packets"]
    #[inline(always)]
    pub fn advint(&self) -> ADVINT_R {
        ADVINT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEADVTIM")
            .field("advint", &self.advint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Time in us between sent advertisement packets"]
    #[inline(always)]
    pub fn advint(&mut self) -> ADVINT_W<'_, BLEADVTIM_SPEC> {
        ADVINT_W::new(self, 0)
    }
}
#[doc = "Advertising Packet Interval\n\nYou can [`read`](crate::Reg::read) this register and get [`bleadvtim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleadvtim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEADVTIM_SPEC;
impl crate::RegisterSpec for BLEADVTIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleadvtim::R`](R) reader structure"]
impl crate::Readable for BLEADVTIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleadvtim::W`](W) writer structure"]
impl crate::Writable for BLEADVTIM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEADVTIM to value 0"]
impl crate::Resettable for BLEADVTIM_SPEC {}
