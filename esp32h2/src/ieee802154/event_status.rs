///Register `EVENT_STATUS` reader
pub type R = crate::R<EVENT_STATUS_SPEC>;
///Register `EVENT_STATUS` writer
pub type W = crate::W<EVENT_STATUS_SPEC>;
///Field `EVENT_STATUS` reader -
pub type EVENT_STATUS_R = crate::FieldReader<u16>;
///Field `EVENT_STATUS` writer -
pub type EVENT_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12
    #[inline(always)]
    pub fn event_status(&self) -> EVENT_STATUS_R {
        EVENT_STATUS_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENT_STATUS")
            .field("event_status", &self.event_status())
            .finish()
    }
}
impl W {
    ///Bits 0:12
    #[inline(always)]
    #[must_use]
    pub fn event_status(&mut self) -> EVENT_STATUS_W<EVENT_STATUS_SPEC> {
        EVENT_STATUS_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`event_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVENT_STATUS_SPEC;
impl crate::RegisterSpec for EVENT_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`event_status::R`](R) reader structure
impl crate::Readable for EVENT_STATUS_SPEC {}
///`write(|w| ..)` method takes [`event_status::W`](W) writer structure
impl crate::Writable for EVENT_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVENT_STATUS to value 0
impl crate::Resettable for EVENT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
