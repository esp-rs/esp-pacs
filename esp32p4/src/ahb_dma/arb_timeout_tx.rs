#[doc = "Register `ARB_TIMEOUT_TX` reader"]
pub type R = crate::R<ARB_TIMEOUT_TX_SPEC>;
#[doc = "Register `ARB_TIMEOUT_TX` writer"]
pub type W = crate::W<ARB_TIMEOUT_TX_SPEC>;
#[doc = "Field `ARB_TIMEOUT_TX` reader - This register is used to config arbiter time out value"]
pub type ARB_TIMEOUT_TX_R = crate::FieldReader<u16>;
#[doc = "Field `ARB_TIMEOUT_TX` writer - This register is used to config arbiter time out value"]
pub type ARB_TIMEOUT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to config arbiter time out value"]
    #[inline(always)]
    pub fn arb_timeout_tx(&self) -> ARB_TIMEOUT_TX_R {
        ARB_TIMEOUT_TX_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_TIMEOUT_TX")
            .field("arb_timeout_tx", &self.arb_timeout_tx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to config arbiter time out value"]
    #[inline(always)]
    pub fn arb_timeout_tx(&mut self) -> ARB_TIMEOUT_TX_W<ARB_TIMEOUT_TX_SPEC> {
        ARB_TIMEOUT_TX_W::new(self, 0)
    }
}
#[doc = "This retister is used to config arbiter time slice for tx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_TIMEOUT_TX_SPEC;
impl crate::RegisterSpec for ARB_TIMEOUT_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_timeout_tx::R`](R) reader structure"]
impl crate::Readable for ARB_TIMEOUT_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_timeout_tx::W`](W) writer structure"]
impl crate::Writable for ARB_TIMEOUT_TX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_TIMEOUT_TX to value 0"]
impl crate::Resettable for ARB_TIMEOUT_TX_SPEC {
    const RESET_VALUE: u32 = 0;
}
