///Register `CHANNEL` reader
pub type R = crate::R<CHANNEL_SPEC>;
///Register `CHANNEL` writer
pub type W = crate::W<CHANNEL_SPEC>;
///Field `HOP` reader -
pub type HOP_R = crate::FieldReader;
///Field `HOP` writer -
pub type HOP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn hop(&self) -> HOP_R {
        HOP_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANNEL").field("hop", &self.hop()).finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn hop(&mut self) -> HOP_W<CHANNEL_SPEC> {
        HOP_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`channel::R`](R) reader structure
impl crate::Readable for CHANNEL_SPEC {}
///`write(|w| ..)` method takes [`channel::W`](W) writer structure
impl crate::Writable for CHANNEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHANNEL to value 0
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
