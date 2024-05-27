///Register `ARB_TIMEOUT` reader
pub type R = crate::R<ARB_TIMEOUT_SPEC>;
///Register `ARB_TIMEOUT` writer
pub type W = crate::W<ARB_TIMEOUT_SPEC>;
///Field `TX` reader - This register is used to config tx arbiter time out value
pub type TX_R = crate::FieldReader<u16>;
///Field `TX` writer - This register is used to config tx arbiter time out value
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RX` reader - This register is used to config rx arbiter time out value
pub type RX_R = crate::FieldReader<u16>;
///Field `RX` writer - This register is used to config rx arbiter time out value
pub type RX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - This register is used to config tx arbiter time out value
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - This register is used to config rx arbiter time out value
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_TIMEOUT")
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - This register is used to config tx arbiter time out value
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<ARB_TIMEOUT_SPEC> {
        TX_W::new(self, 0)
    }
    ///Bits 16:31 - This register is used to config rx arbiter time out value
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<ARB_TIMEOUT_SPEC> {
        RX_W::new(self, 16)
    }
}
/**This retister is used to config arbiter time slice

You can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ARB_TIMEOUT_SPEC;
impl crate::RegisterSpec for ARB_TIMEOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`arb_timeout::R`](R) reader structure
impl crate::Readable for ARB_TIMEOUT_SPEC {}
///`write(|w| ..)` method takes [`arb_timeout::W`](W) writer structure
impl crate::Writable for ARB_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ARB_TIMEOUT to value 0
impl crate::Resettable for ARB_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
