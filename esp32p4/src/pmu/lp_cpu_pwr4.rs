#[doc = "Register `LP_CPU_PWR4` reader"]
pub type R = crate::R<LP_CPU_PWR4_SPEC>;
#[doc = "Register `LP_CPU_PWR4` writer"]
pub type W = crate::W<LP_CPU_PWR4_SPEC>;
#[doc = "Field `LP_CPU_REJECT_EN` reader - need_des"]
pub type LP_CPU_REJECT_EN_R = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_REJECT_EN` writer - need_des"]
pub type LP_CPU_REJECT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_reject_en(&self) -> LP_CPU_REJECT_EN_R {
        LP_CPU_REJECT_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_PWR4")
            .field(
                "lp_cpu_reject_en",
                &format_args!("{}", self.lp_cpu_reject_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CPU_PWR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_reject_en(&mut self) -> LP_CPU_REJECT_EN_W<LP_CPU_PWR4_SPEC> {
        LP_CPU_REJECT_EN_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CPU_PWR4_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr4::R`](R) reader structure"]
impl crate::Readable for LP_CPU_PWR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_cpu_pwr4::W`](W) writer structure"]
impl crate::Writable for LP_CPU_PWR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CPU_PWR4 to value 0"]
impl crate::Resettable for LP_CPU_PWR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
