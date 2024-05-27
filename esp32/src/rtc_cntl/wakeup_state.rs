#[doc = "Register `WAKEUP_STATE` reader"]
pub type R = crate::R<WAKEUP_STATE_SPEC>;
#[doc = "Register `WAKEUP_STATE` writer"]
pub type W = crate::W<WAKEUP_STATE_SPEC>;
#[doc = "Field `WAKEUP_CAUSE` reader - wakeup cause"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u16>;
#[doc = "Field `WAKEUP_ENA` reader - wakeup enable bitmap"]
pub type WAKEUP_ENA_R = crate::FieldReader<u16>;
#[doc = "Field `WAKEUP_ENA` writer - wakeup enable bitmap"]
pub type WAKEUP_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_STATE")
            .field("wakeup_cause", &self.wakeup_cause())
            .field("wakeup_ena", &self.wakeup_ena())
            .field("gpio_wakeup_filter", &self.gpio_wakeup_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W<WAKEUP_STATE_SPEC> {
        WAKEUP_ENA_W::new(self, 11)
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W<WAKEUP_STATE_SPEC> {
        GPIO_WAKEUP_FILTER_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_STATE_SPEC;
impl crate::RegisterSpec for WAKEUP_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_state::R`](R) reader structure"]
impl crate::Readable for WAKEUP_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_state::W`](W) writer structure"]
impl crate::Writable for WAKEUP_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_STATE to value 0x6000"]
impl crate::Resettable for WAKEUP_STATE_SPEC {
    const RESET_VALUE: u32 = 0x6000;
}
