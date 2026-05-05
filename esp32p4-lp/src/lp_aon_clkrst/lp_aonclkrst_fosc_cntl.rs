#[doc = "Register `LP_AONCLKRST_FOSC_CNTL` reader"]
pub type R = crate::R<LP_AONCLKRST_FOSC_CNTL_SPEC>;
#[doc = "Register `LP_AONCLKRST_FOSC_CNTL` writer"]
pub type W = crate::W<LP_AONCLKRST_FOSC_CNTL_SPEC>;
#[doc = "Field `LP_AONCLKRST_FOSC_DFREQ` reader - need_des"]
pub type LP_AONCLKRST_FOSC_DFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `LP_AONCLKRST_FOSC_DFREQ` writer - need_des"]
pub type LP_AONCLKRST_FOSC_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_dfreq(&self) -> LP_AONCLKRST_FOSC_DFREQ_R {
        LP_AONCLKRST_FOSC_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_FOSC_CNTL")
            .field("lp_aonclkrst_fosc_dfreq", &self.lp_aonclkrst_fosc_dfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_dfreq(
        &mut self,
    ) -> LP_AONCLKRST_FOSC_DFREQ_W<'_, LP_AONCLKRST_FOSC_CNTL_SPEC> {
        LP_AONCLKRST_FOSC_DFREQ_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_fosc_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_fosc_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_FOSC_CNTL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_FOSC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_fosc_cntl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_FOSC_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_fosc_cntl::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_FOSC_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_FOSC_CNTL to value 0x6400_0000"]
impl crate::Resettable for LP_AONCLKRST_FOSC_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x6400_0000;
}
