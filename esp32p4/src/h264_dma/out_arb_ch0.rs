#[doc = "Register `OUT_ARB_CH0` reader"]
pub type R = crate::R<OUT_ARB_CH0_SPEC>;
#[doc = "Register `OUT_ARB_CH0` writer"]
pub type W = crate::W<OUT_ARB_CH0_SPEC>;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH0` reader - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH0_R = crate::FieldReader;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH0` writer - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTER_OUT_ARB_PRIORITY_CH0` reader - Set the priority of channel"]
pub type EXTER_OUT_ARB_PRIORITY_CH0_R = crate::FieldReader;
#[doc = "Field `EXTER_OUT_ARB_PRIORITY_CH0` writer - Set the priority of channel"]
pub type EXTER_OUT_ARB_PRIORITY_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn out_arb_token_num_ch0(&self) -> OUT_ARB_TOKEN_NUM_CH0_R {
        OUT_ARB_TOKEN_NUM_CH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Set the priority of channel"]
    #[inline(always)]
    pub fn exter_out_arb_priority_ch0(&self) -> EXTER_OUT_ARB_PRIORITY_CH0_R {
        EXTER_OUT_ARB_PRIORITY_CH0_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ARB_CH0")
            .field(
                "out_arb_token_num_ch0",
                &format_args!("{}", self.out_arb_token_num_ch0().bits()),
            )
            .field(
                "exter_out_arb_priority_ch0",
                &format_args!("{}", self.exter_out_arb_priority_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ARB_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn out_arb_token_num_ch0(&mut self) -> OUT_ARB_TOKEN_NUM_CH0_W<OUT_ARB_CH0_SPEC> {
        OUT_ARB_TOKEN_NUM_CH0_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn exter_out_arb_priority_ch0(&mut self) -> EXTER_OUT_ARB_PRIORITY_CH0_W<OUT_ARB_CH0_SPEC> {
        EXTER_OUT_ARB_PRIORITY_CH0_W::new(self, 4)
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
#[doc = "TX CH0 arb register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_arb_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_arb_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ARB_CH0_SPEC;
impl crate::RegisterSpec for OUT_ARB_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_arb_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_ARB_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_arb_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_ARB_CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_ARB_CH0 to value 0x11"]
impl crate::Resettable for OUT_ARB_CH0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
