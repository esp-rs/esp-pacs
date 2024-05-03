#[doc = "Register `ARB_TIMEOUT` reader"]
pub type R = crate::R<ARB_TIMEOUT_SPEC>;
#[doc = "Register `ARB_TIMEOUT` writer"]
pub type W = crate::W<ARB_TIMEOUT_SPEC>;
#[doc = "Field `TX` reader - This register is used to config tx arbiter time out value"]
pub type TX_R = crate::FieldReader<u16>;
#[doc = "Field `TX` writer - This register is used to config tx arbiter time out value"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX` reader - This register is used to config rx arbiter time out value"]
pub type RX_R = crate::FieldReader<u16>;
#[doc = "Field `RX` writer - This register is used to config rx arbiter time out value"]
pub type RX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to config tx arbiter time out value"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to config rx arbiter time out value"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_TIMEOUT")
            .field("tx", &self.tx().bits())
            .field("rx", &self.rx().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ARB_TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to config tx arbiter time out value"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<ARB_TIMEOUT_SPEC> {
        TX_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This register is used to config rx arbiter time out value"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<ARB_TIMEOUT_SPEC> {
        RX_W::new(self, 16)
    }
}
#[doc = "This retister is used to config arbiter time slice\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_TIMEOUT_SPEC;
impl crate::RegisterSpec for ARB_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_timeout::R`](R) reader structure"]
impl crate::Readable for ARB_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_timeout::W`](W) writer structure"]
impl crate::Writable for ARB_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_TIMEOUT to value 0"]
impl crate::Resettable for ARB_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
