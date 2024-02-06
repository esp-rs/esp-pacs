#[doc = "Register `OUT_ARB_CH1` reader"]
pub type R = crate::R<OUT_ARB_CH1_SPEC>;
#[doc = "Register `OUT_ARB_CH1` writer"]
pub type W = crate::W<OUT_ARB_CH1_SPEC>;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH1` reader - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH1_R = crate::FieldReader;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH1` writer - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTER_OUT_ARB_PRIORITY_CH1` reader - Set the priority of channel"]
pub type INTER_OUT_ARB_PRIORITY_CH1_R = crate::BitReader;
#[doc = "Field `INTER_OUT_ARB_PRIORITY_CH1` writer - Set the priority of channel"]
pub type INTER_OUT_ARB_PRIORITY_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn out_arb_token_num_ch1(&self) -> OUT_ARB_TOKEN_NUM_CH1_R {
        OUT_ARB_TOKEN_NUM_CH1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_out_arb_priority_ch1(&self) -> INTER_OUT_ARB_PRIORITY_CH1_R {
        INTER_OUT_ARB_PRIORITY_CH1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ARB_CH1")
            .field(
                "out_arb_token_num_ch1",
                &format_args!("{}", self.out_arb_token_num_ch1().bits()),
            )
            .field(
                "inter_out_arb_priority_ch1",
                &format_args!("{}", self.inter_out_arb_priority_ch1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ARB_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn out_arb_token_num_ch1(&mut self) -> OUT_ARB_TOKEN_NUM_CH1_W<OUT_ARB_CH1_SPEC> {
        OUT_ARB_TOKEN_NUM_CH1_W::new(self, 0)
    }
    #[doc = "Bit 6 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn inter_out_arb_priority_ch1(&mut self) -> INTER_OUT_ARB_PRIORITY_CH1_W<OUT_ARB_CH1_SPEC> {
        INTER_OUT_ARB_PRIORITY_CH1_W::new(self, 6)
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
#[doc = "TX CH1 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ARB_CH1_SPEC;
impl crate::RegisterSpec for OUT_ARB_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_arb_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_ARB_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_arb_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_ARB_CH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_ARB_CH1 to value 0x41"]
impl crate::Resettable for OUT_ARB_CH1_SPEC {
    const RESET_VALUE: u32 = 0x41;
}
