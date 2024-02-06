#[doc = "Register `STORE5` reader"]
pub type R = crate::R<STORE5_SPEC>;
#[doc = "Register `STORE5` writer"]
pub type W = crate::W<STORE5_SPEC>;
#[doc = "Field `LP_AON_STORE5` reader - need_des"]
pub type LP_AON_STORE5_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE5` writer - need_des"]
pub type LP_AON_STORE5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store5(&self) -> LP_AON_STORE5_R {
        LP_AON_STORE5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE5")
            .field(
                "lp_aon_store5",
                &format_args!("{}", self.lp_aon_store5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store5(&mut self) -> LP_AON_STORE5_W<STORE5_SPEC> {
        LP_AON_STORE5_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE5_SPEC;
impl crate::RegisterSpec for STORE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store5::R`](R) reader structure"]
impl crate::Readable for STORE5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store5::W`](W) writer structure"]
impl crate::Writable for STORE5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE5 to value 0"]
impl crate::Resettable for STORE5_SPEC {
    const RESET_VALUE: u32 = 0;
}
