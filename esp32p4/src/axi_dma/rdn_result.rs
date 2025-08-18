#[doc = "Register `RDN_RESULT` reader"]
pub type R = crate::R<RDN_RESULT_SPEC>;
#[doc = "Register `RDN_RESULT` writer"]
pub type W = crate::W<RDN_RESULT_SPEC>;
#[doc = "Field `RDN_ENA` reader - reserved"]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - reserved"]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - reserved"]
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_RESULT")
            .field("rdn_ena", &self.rdn_ena())
            .field("rdn_result", &self.rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<'_, RDN_RESULT_SPEC> {
        RDN_ENA_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDN_RESULT_SPEC;
impl crate::RegisterSpec for RDN_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_result::R`](R) reader structure"]
impl crate::Readable for RDN_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdn_result::W`](W) writer structure"]
impl crate::Writable for RDN_RESULT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDN_RESULT to value 0"]
impl crate::Resettable for RDN_RESULT_SPEC {}
