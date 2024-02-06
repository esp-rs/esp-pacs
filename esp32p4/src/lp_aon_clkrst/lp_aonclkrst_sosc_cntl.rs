#[doc = "Register `LP_AONCLKRST_SOSC_CNTL` reader"]
pub type R = crate::R<LP_AONCLKRST_SOSC_CNTL_SPEC>;
#[doc = "Register `LP_AONCLKRST_SOSC_CNTL` writer"]
pub type W = crate::W<LP_AONCLKRST_SOSC_CNTL_SPEC>;
#[doc = "Field `LP_AONCLKRST_SOSC_DFREQ` reader - need_des"]
pub type LP_AONCLKRST_SOSC_DFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `LP_AONCLKRST_SOSC_DFREQ` writer - need_des"]
pub type LP_AONCLKRST_SOSC_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_sosc_dfreq(&self) -> LP_AONCLKRST_SOSC_DFREQ_R {
        LP_AONCLKRST_SOSC_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_SOSC_CNTL")
            .field(
                "lp_aonclkrst_sosc_dfreq",
                &format_args!("{}", self.lp_aonclkrst_sosc_dfreq().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_SOSC_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_sosc_dfreq(
        &mut self,
    ) -> LP_AONCLKRST_SOSC_DFREQ_W<LP_AONCLKRST_SOSC_CNTL_SPEC> {
        LP_AONCLKRST_SOSC_DFREQ_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_sosc_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_sosc_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_SOSC_CNTL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_SOSC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_sosc_cntl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_SOSC_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_sosc_cntl::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_SOSC_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_SOSC_CNTL to value 0x2b00_0000"]
impl crate::Resettable for LP_AONCLKRST_SOSC_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x2b00_0000;
}
