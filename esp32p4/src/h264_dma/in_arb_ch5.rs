#[doc = "Register `IN_ARB_CH5` reader"]
pub type R = crate::R<IN_ARB_CH5_SPEC>;
#[doc = "Register `IN_ARB_CH5` writer"]
pub type W = crate::W<IN_ARB_CH5_SPEC>;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH5` reader - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH5_R = crate::FieldReader;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH5` writer - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTER_IN_ARB_PRIORITY_CH5` reader - Set the priority of channel"]
pub type INTER_IN_ARB_PRIORITY_CH5_R = crate::FieldReader;
#[doc = "Field `INTER_IN_ARB_PRIORITY_CH5` writer - Set the priority of channel"]
pub type INTER_IN_ARB_PRIORITY_CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num_ch5(&self) -> IN_ARB_TOKEN_NUM_CH5_R {
        IN_ARB_TOKEN_NUM_CH5_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_in_arb_priority_ch5(&self) -> INTER_IN_ARB_PRIORITY_CH5_R {
        INTER_IN_ARB_PRIORITY_CH5_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ARB_CH5")
            .field(
                "in_arb_token_num_ch5",
                &format_args!("{}", self.in_arb_token_num_ch5().bits()),
            )
            .field(
                "inter_in_arb_priority_ch5",
                &format_args!("{}", self.inter_in_arb_priority_ch5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_ARB_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn in_arb_token_num_ch5(&mut self) -> IN_ARB_TOKEN_NUM_CH5_W<IN_ARB_CH5_SPEC> {
        IN_ARB_TOKEN_NUM_CH5_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn inter_in_arb_priority_ch5(&mut self) -> INTER_IN_ARB_PRIORITY_CH5_W<IN_ARB_CH5_SPEC> {
        INTER_IN_ARB_PRIORITY_CH5_W::new(self, 6)
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
#[doc = "RX CH5 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_arb_ch5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_arb_ch5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ARB_CH5_SPEC;
impl crate::RegisterSpec for IN_ARB_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_arb_ch5::R`](R) reader structure"]
impl crate::Readable for IN_ARB_CH5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_arb_ch5::W`](W) writer structure"]
impl crate::Writable for IN_ARB_CH5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_ARB_CH5 to value 0x41"]
impl crate::Resettable for IN_ARB_CH5_SPEC {
    const RESET_VALUE: Self::Ux = 0x41;
}
