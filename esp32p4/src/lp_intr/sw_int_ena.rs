#[doc = "Register `SW_INT_ENA` reader"]
pub type R = crate::R<SW_INT_ENA_SPEC>;
#[doc = "Register `SW_INT_ENA` writer"]
pub type W = crate::W<SW_INT_ENA_SPEC>;
#[doc = "Field `LP_SW_INT_ENA` reader - need_des"]
pub type LP_SW_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_SW_INT_ENA` writer - need_des"]
pub type LP_SW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sw_int_ena(&self) -> LP_SW_INT_ENA_R {
        LP_SW_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_INT_ENA")
            .field(
                "lp_sw_int_ena",
                &format_args!("{}", self.lp_sw_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sw_int_ena(&mut self) -> LP_SW_INT_ENA_W<SW_INT_ENA_SPEC> {
        LP_SW_INT_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_INT_ENA_SPEC;
impl crate::RegisterSpec for SW_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_int_ena::R`](R) reader structure"]
impl crate::Readable for SW_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_int_ena::W`](W) writer structure"]
impl crate::Writable for SW_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_INT_ENA to value 0"]
impl crate::Resettable for SW_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
