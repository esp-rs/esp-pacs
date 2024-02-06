#[doc = "Register `IN_ARB_CONFIG` reader"]
pub type R = crate::R<IN_ARB_CONFIG_SPEC>;
#[doc = "Register `IN_ARB_CONFIG` writer"]
pub type W = crate::W<IN_ARB_CONFIG_SPEC>;
#[doc = "Field `IN_ARB_TIMEOUT_NUM` reader - Set the max number of timeout count of arbiter"]
pub type IN_ARB_TIMEOUT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `IN_ARB_TIMEOUT_NUM` writer - Set the max number of timeout count of arbiter"]
pub type IN_ARB_TIMEOUT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IN_WEIGHT_EN` reader - reserved"]
pub type IN_WEIGHT_EN_R = crate::BitReader;
#[doc = "Field `IN_WEIGHT_EN` writer - reserved"]
pub type IN_WEIGHT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Set the max number of timeout count of arbiter"]
    #[inline(always)]
    pub fn in_arb_timeout_num(&self) -> IN_ARB_TIMEOUT_NUM_R {
        IN_ARB_TIMEOUT_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn in_weight_en(&self) -> IN_WEIGHT_EN_R {
        IN_WEIGHT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ARB_CONFIG")
            .field(
                "in_arb_timeout_num",
                &format_args!("{}", self.in_arb_timeout_num().bits()),
            )
            .field(
                "in_weight_en",
                &format_args!("{}", self.in_weight_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_ARB_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the max number of timeout count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn in_arb_timeout_num(&mut self) -> IN_ARB_TIMEOUT_NUM_W<IN_ARB_CONFIG_SPEC> {
        IN_ARB_TIMEOUT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn in_weight_en(&mut self) -> IN_WEIGHT_EN_W<IN_ARB_CONFIG_SPEC> {
        IN_WEIGHT_EN_W::new(self, 16)
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
#[doc = "reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ARB_CONFIG_SPEC;
impl crate::RegisterSpec for IN_ARB_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_arb_config::R`](R) reader structure"]
impl crate::Readable for IN_ARB_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_arb_config::W`](W) writer structure"]
impl crate::Writable for IN_ARB_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_ARB_CONFIG to value 0"]
impl crate::Resettable for IN_ARB_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
