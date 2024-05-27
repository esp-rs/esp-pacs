///Register `SLEEP_CONF` reader
pub type R = crate::R<SLEEP_CONF_SPEC>;
///Register `SLEEP_CONF` writer
pub type W = crate::W<SLEEP_CONF_SPEC>;
///Field `ACTIVE_THRESHOLD` reader - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode.
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16>;
///Field `ACTIVE_THRESHOLD` writer - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode.
pub type ACTIVE_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode.
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF")
            .field("active_threshold", &self.active_threshold())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode.
    #[inline(always)]
    #[must_use]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W<SLEEP_CONF_SPEC> {
        ACTIVE_THRESHOLD_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLEEP_CONF_SPEC;
impl crate::RegisterSpec for SLEEP_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sleep_conf::R`](R) reader structure
impl crate::Readable for SLEEP_CONF_SPEC {}
///`write(|w| ..)` method takes [`sleep_conf::W`](W) writer structure
impl crate::Writable for SLEEP_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLEEP_CONF to value 0xf0
impl crate::Resettable for SLEEP_CONF_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
