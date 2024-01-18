#[doc = "Register `PERI1_APB_POSTW_EN` reader"]
pub type R = crate::R<PERI1_APB_POSTW_EN_SPEC>;
#[doc = "Register `PERI1_APB_POSTW_EN` writer"]
pub type W = crate::W<PERI1_APB_POSTW_EN_SPEC>;
#[doc = "Field `PERI1_APB_POSTW_EN` reader - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register"]
pub type PERI1_APB_POSTW_EN_R = crate::BitReader;
#[doc = "Field `PERI1_APB_POSTW_EN` writer - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register"]
pub type PERI1_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register"]
    #[inline(always)]
    pub fn peri1_apb_postw_en(&self) -> PERI1_APB_POSTW_EN_R {
        PERI1_APB_POSTW_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI1_APB_POSTW_EN")
            .field(
                "peri1_apb_postw_en",
                &format_args!("{}", self.peri1_apb_postw_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI1_APB_POSTW_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register"]
    #[inline(always)]
    #[must_use]
    pub fn peri1_apb_postw_en(&mut self) -> PERI1_APB_POSTW_EN_W<PERI1_APB_POSTW_EN_SPEC> {
        PERI1_APB_POSTW_EN_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri1_apb_postw_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri1_apb_postw_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI1_APB_POSTW_EN_SPEC;
impl crate::RegisterSpec for PERI1_APB_POSTW_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri1_apb_postw_en::R`](R) reader structure"]
impl crate::Readable for PERI1_APB_POSTW_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri1_apb_postw_en::W`](W) writer structure"]
impl crate::Writable for PERI1_APB_POSTW_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI1_APB_POSTW_EN to value 0"]
impl crate::Resettable for PERI1_APB_POSTW_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
