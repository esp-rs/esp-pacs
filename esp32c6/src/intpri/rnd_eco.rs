#[doc = "Register `RND_ECO` reader"]
pub type R = crate::R<RND_ECO_SPEC>;
#[doc = "Register `RND_ECO` writer"]
pub type W = crate::W<RND_ECO_SPEC>;
#[doc = "Field `REDCY_ENA` reader - Only reserved for ECO."]
pub type REDCY_ENA_R = crate::BitReader;
#[doc = "Field `REDCY_ENA` writer - Only reserved for ECO."]
pub type REDCY_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDCY_RESULT` reader - Only reserved for ECO."]
pub type REDCY_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_ena(&self) -> REDCY_ENA_R {
        REDCY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_result(&self) -> REDCY_RESULT_R {
        REDCY_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO")
            .field("redcy_ena", &format_args!("{}", self.redcy_ena().bit()))
            .field(
                "redcy_result",
                &format_args!("{}", self.redcy_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RND_ECO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Only reserved for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn redcy_ena(&mut self) -> REDCY_ENA_W<RND_ECO_SPEC> {
        REDCY_ENA_W::new(self, 0)
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
#[doc = "redcy eco register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RND_ECO_SPEC;
impl crate::RegisterSpec for RND_ECO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_eco::R`](R) reader structure"]
impl crate::Readable for RND_ECO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rnd_eco::W`](W) writer structure"]
impl crate::Writable for RND_ECO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RND_ECO to value 0"]
impl crate::Resettable for RND_ECO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
