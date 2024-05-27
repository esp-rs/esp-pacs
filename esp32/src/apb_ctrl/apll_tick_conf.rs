#[doc = "Register `APLL_TICK_CONF` reader"]
pub type R = crate::R<APLL_TICK_CONF_SPEC>;
#[doc = "Register `APLL_TICK_CONF` writer"]
pub type W = crate::W<APLL_TICK_CONF_SPEC>;
#[doc = "Field `APLL_TICK_NUM` reader - "]
pub type APLL_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `APLL_TICK_NUM` writer - "]
pub type APLL_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apll_tick_num(&self) -> APLL_TICK_NUM_R {
        APLL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APLL_TICK_CONF")
            .field("apll_tick_num", &self.apll_tick_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn apll_tick_num(&mut self) -> APLL_TICK_NUM_W<APLL_TICK_CONF_SPEC> {
        APLL_TICK_NUM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apll_tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apll_tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APLL_TICK_CONF_SPEC;
impl crate::RegisterSpec for APLL_TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apll_tick_conf::R`](R) reader structure"]
impl crate::Readable for APLL_TICK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apll_tick_conf::W`](W) writer structure"]
impl crate::Writable for APLL_TICK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APLL_TICK_CONF to value 0x63"]
impl crate::Resettable for APLL_TICK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x63;
}
