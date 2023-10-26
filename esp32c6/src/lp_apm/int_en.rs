#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `M0_APM_INT_EN` reader - APM M0 interrupt enable"]
pub type M0_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M0_APM_INT_EN` writer - APM M0 interrupt enable"]
pub type M0_APM_INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `M1_APM_INT_EN` reader - APM M1 interrupt enable"]
pub type M1_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M1_APM_INT_EN` writer - APM M1 interrupt enable"]
pub type M1_APM_INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - APM M0 interrupt enable"]
    #[inline(always)]
    pub fn m0_apm_int_en(&self) -> M0_APM_INT_EN_R {
        M0_APM_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APM M1 interrupt enable"]
    #[inline(always)]
    pub fn m1_apm_int_en(&self) -> M1_APM_INT_EN_R {
        M1_APM_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_EN")
            .field(
                "m0_apm_int_en",
                &format_args!("{}", self.m0_apm_int_en().bit()),
            )
            .field(
                "m1_apm_int_en",
                &format_args!("{}", self.m1_apm_int_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - APM M0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0_apm_int_en(&mut self) -> M0_APM_INT_EN_W<INT_EN_SPEC, 0> {
        M0_APM_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - APM M1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1_apm_int_en(&mut self) -> M1_APM_INT_EN_W<INT_EN_SPEC, 1> {
        M1_APM_INT_EN_W::new(self)
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
#[doc = "APM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
