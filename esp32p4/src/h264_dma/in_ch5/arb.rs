///Register `ARB` reader
pub type R = crate::R<ARB_SPEC>;
///Register `ARB` writer
pub type W = crate::W<ARB_SPEC>;
///Field `IN_ARB_TOKEN_NUM` reader - Set the max number of token count of arbiter
pub type IN_ARB_TOKEN_NUM_R = crate::FieldReader;
///Field `IN_ARB_TOKEN_NUM` writer - Set the max number of token count of arbiter
pub type IN_ARB_TOKEN_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `INTER_IN_ARB_PRIORITY` reader - Set the priority of channel
pub type INTER_IN_ARB_PRIORITY_R = crate::FieldReader;
///Field `INTER_IN_ARB_PRIORITY` writer - Set the priority of channel
pub type INTER_IN_ARB_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - Set the max number of token count of arbiter
    #[inline(always)]
    pub fn in_arb_token_num(&self) -> IN_ARB_TOKEN_NUM_R {
        IN_ARB_TOKEN_NUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:8 - Set the priority of channel
    #[inline(always)]
    pub fn inter_in_arb_priority(&self) -> INTER_IN_ARB_PRIORITY_R {
        INTER_IN_ARB_PRIORITY_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB")
            .field("in_arb_token_num", &self.in_arb_token_num())
            .field("inter_in_arb_priority", &self.inter_in_arb_priority())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Set the max number of token count of arbiter
    #[inline(always)]
    #[must_use]
    pub fn in_arb_token_num(&mut self) -> IN_ARB_TOKEN_NUM_W<ARB_SPEC> {
        IN_ARB_TOKEN_NUM_W::new(self, 0)
    }
    ///Bits 6:8 - Set the priority of channel
    #[inline(always)]
    #[must_use]
    pub fn inter_in_arb_priority(&mut self) -> INTER_IN_ARB_PRIORITY_W<ARB_SPEC> {
        INTER_IN_ARB_PRIORITY_W::new(self, 6)
    }
}
/**RX CH5 arb register

You can [`read`](crate::generic::Reg::read) this register and get [`arb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ARB_SPEC;
impl crate::RegisterSpec for ARB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`arb::R`](R) reader structure
impl crate::Readable for ARB_SPEC {}
///`write(|w| ..)` method takes [`arb::W`](W) writer structure
impl crate::Writable for ARB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ARB to value 0x41
impl crate::Resettable for ARB_SPEC {
    const RESET_VALUE: u32 = 0x41;
}
