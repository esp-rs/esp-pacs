#[doc = "Register `ARB` reader"]
pub type R = crate::R<ARB_SPEC>;
#[doc = "Register `ARB` writer"]
pub type W = crate::W<ARB_SPEC>;
#[doc = "Field `OUT_ARB_TOKEN_NUM` reader - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_R = crate::FieldReader;
#[doc = "Field `OUT_ARB_TOKEN_NUM` writer - Set the max number of token count of arbiter"]
pub type OUT_ARB_TOKEN_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTER_OUT_ARB_PRIORITY` reader - Set the priority of channel"]
pub type EXTER_OUT_ARB_PRIORITY_R = crate::FieldReader;
#[doc = "Field `EXTER_OUT_ARB_PRIORITY` writer - Set the priority of channel"]
pub type EXTER_OUT_ARB_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTER_OUT_ARB_PRIORITY` reader - Set the priority of channel"]
pub type INTER_OUT_ARB_PRIORITY_R = crate::BitReader;
#[doc = "Field `INTER_OUT_ARB_PRIORITY` writer - Set the priority of channel"]
pub type INTER_OUT_ARB_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    pub fn out_arb_token_num(&self) -> OUT_ARB_TOKEN_NUM_R {
        OUT_ARB_TOKEN_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Set the priority of channel"]
    #[inline(always)]
    pub fn exter_out_arb_priority(&self) -> EXTER_OUT_ARB_PRIORITY_R {
        EXTER_OUT_ARB_PRIORITY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set the priority of channel"]
    #[inline(always)]
    pub fn inter_out_arb_priority(&self) -> INTER_OUT_ARB_PRIORITY_R {
        INTER_OUT_ARB_PRIORITY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB")
            .field("out_arb_token_num", &self.out_arb_token_num())
            .field("exter_out_arb_priority", &self.exter_out_arb_priority())
            .field("inter_out_arb_priority", &self.inter_out_arb_priority())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Set the max number of token count of arbiter"]
    #[inline(always)]
    #[must_use]
    pub fn out_arb_token_num(&mut self) -> OUT_ARB_TOKEN_NUM_W<ARB_SPEC> {
        OUT_ARB_TOKEN_NUM_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn exter_out_arb_priority(&mut self) -> EXTER_OUT_ARB_PRIORITY_W<ARB_SPEC> {
        EXTER_OUT_ARB_PRIORITY_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set the priority of channel"]
    #[inline(always)]
    #[must_use]
    pub fn inter_out_arb_priority(&mut self) -> INTER_OUT_ARB_PRIORITY_W<ARB_SPEC> {
        INTER_OUT_ARB_PRIORITY_W::new(self, 6)
    }
}
#[doc = "TX CHx arb register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_SPEC;
impl crate::RegisterSpec for ARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb::R`](R) reader structure"]
impl crate::Readable for ARB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb::W`](W) writer structure"]
impl crate::Writable for ARB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB to value 0x11"]
impl crate::Resettable for ARB_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
