#[doc = "Register `IN_ARB_CH%s` reader"]
pub type R = crate::R<IN_ARB_CH_SPEC>;
#[doc = "Register `IN_ARB_CH%s` writer"]
pub type W = crate::W<IN_ARB_CH_SPEC>;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH` reader - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH_R = crate::FieldReader;
#[doc = "Field `IN_ARB_TOKEN_NUM_CH` writer - Set the max number of token count of arbiter"]
pub type IN_ARB_TOKEN_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IN_ARB_PRIORITY_CH` reader - Set the priority of channel"]
pub type IN_ARB_PRIORITY_CH_R = crate::FieldReader;
#[doc = "Field `IN_ARB_PRIORITY_CH` writer - Set the priority of channel"]
pub type IN_ARB_PRIORITY_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num_ch(&self) -> IN_ARB_TOKEN_NUM_CH_R {
        IN_ARB_TOKEN_NUM_CH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set the priority of channel"]
    #[inline(always)]
    pub fn in_arb_priority_ch(&self) -> IN_ARB_PRIORITY_CH_R {
        IN_ARB_PRIORITY_CH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ARB_CH")
            .field("in_arb_token_num_ch", &self.in_arb_token_num_ch())
            .field("in_arb_priority_ch", &self.in_arb_priority_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn in_arb_token_num_ch(&mut self) -> IN_ARB_TOKEN_NUM_CH_W<'_, IN_ARB_CH_SPEC> {
        IN_ARB_TOKEN_NUM_CH_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set the priority of channel"]
    #[inline(always)]
    pub fn in_arb_priority_ch(&mut self) -> IN_ARB_PRIORITY_CH_W<'_, IN_ARB_CH_SPEC> {
        IN_ARB_PRIORITY_CH_W::new(self, 4)
    }
}
#[doc = "Configures the rx arbiter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_arb_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_arb_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ARB_CH_SPEC;
impl crate::RegisterSpec for IN_ARB_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_arb_ch::R`](R) reader structure"]
impl crate::Readable for IN_ARB_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_arb_ch::W`](W) writer structure"]
impl crate::Writable for IN_ARB_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_ARB_CH%s to value 0"]
impl crate::Resettable for IN_ARB_CH_SPEC {}
