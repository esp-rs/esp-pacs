#[doc = "Register `OUT_ARB_CH%s` reader"]
pub type R = crate::R<OUT_ARB_CH_SPEC>;
#[doc = "Register `OUT_ARB_CH%s` writer"]
pub type W = crate::W<OUT_ARB_CH_SPEC>;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH` reader - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH_R = crate::FieldReader;
#[doc = "Field `OUT_ARB_TOKEN_NUM_CH` writer - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_ARB_PRIORITY_CH` reader - Set the priority of channel"]
pub type OUT_ARB_PRIORITY_CH_R = crate::FieldReader;
#[doc = "Field `OUT_ARB_PRIORITY_CH` writer - Set the priority of channel"]
pub type OUT_ARB_PRIORITY_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn out_arb_token_num_ch(&self) -> OUT_ARB_TOKEN_NUM_CH_R {
        OUT_ARB_TOKEN_NUM_CH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set the priority of channel"]
    #[inline(always)]
    pub fn out_arb_priority_ch(&self) -> OUT_ARB_PRIORITY_CH_R {
        OUT_ARB_PRIORITY_CH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ARB_CH")
            .field("out_arb_token_num_ch", &self.out_arb_token_num_ch())
            .field("out_arb_priority_ch", &self.out_arb_priority_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn out_arb_token_num_ch(&mut self) -> OUT_ARB_TOKEN_NUM_CH_W<'_, OUT_ARB_CH_SPEC> {
        OUT_ARB_TOKEN_NUM_CH_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set the priority of channel"]
    #[inline(always)]
    pub fn out_arb_priority_ch(&mut self) -> OUT_ARB_PRIORITY_CH_W<'_, OUT_ARB_CH_SPEC> {
        OUT_ARB_PRIORITY_CH_W::new(self, 4)
    }
}
#[doc = "Configures the tx arbiter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_arb_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_arb_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ARB_CH_SPEC;
impl crate::RegisterSpec for OUT_ARB_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_arb_ch::R`](R) reader structure"]
impl crate::Readable for OUT_ARB_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_arb_ch::W`](W) writer structure"]
impl crate::Writable for OUT_ARB_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_ARB_CH%s to value 0"]
impl crate::Resettable for OUT_ARB_CH_SPEC {}
