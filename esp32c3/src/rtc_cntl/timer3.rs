#[doc = "Register `TIMER3` reader"]
pub struct R(crate::R<TIMER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER3` writer"]
pub struct W(crate::W<TIMER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_WAIT_TIMER` reader - wifi power domain wakeup time"]
pub type WIFI_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIFI_WAIT_TIMER` writer - wifi power domain wakeup time"]
pub type WIFI_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER3_SPEC, u16, u16, 9, O>;
#[doc = "Field `WIFI_POWERUP_TIMER` reader - wifi power domain power on time"]
pub type WIFI_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIFI_POWERUP_TIMER` writer - wifi power domain power on time"]
pub type WIFI_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER3_SPEC, u8, u8, 7, O>;
#[doc = "Field `BT_WAIT_TIMER` reader - bt power domain wakeup time"]
pub type BT_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BT_WAIT_TIMER` writer - bt power domain wakeup time"]
pub type BT_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER3_SPEC, u16, u16, 9, O>;
#[doc = "Field `BT_POWERUP_TIMER` reader - bt power domain power on time"]
pub type BT_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BT_POWERUP_TIMER` writer - bt power domain power on time"]
pub type BT_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER3_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - wifi power domain wakeup time"]
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - wifi power domain power on time"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - bt power domain wakeup time"]
    #[inline(always)]
    pub fn bt_wait_timer(&self) -> BT_WAIT_TIMER_R {
        BT_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - bt power domain power on time"]
    #[inline(always)]
    pub fn bt_powerup_timer(&self) -> BT_POWERUP_TIMER_R {
        BT_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - wifi power domain wakeup time"]
    #[inline(always)]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W<0> {
        WIFI_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 9:15 - wifi power domain power on time"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W<9> {
        WIFI_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 16:24 - bt power domain wakeup time"]
    #[inline(always)]
    pub fn bt_wait_timer(&mut self) -> BT_WAIT_TIMER_W<16> {
        BT_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31 - bt power domain power on time"]
    #[inline(always)]
    pub fn bt_powerup_timer(&mut self) -> BT_POWERUP_TIMER_W<25> {
        BT_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3](index.html) module"]
pub struct TIMER3_SPEC;
impl crate::RegisterSpec for TIMER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer3::R](R) reader structure"]
impl crate::Readable for TIMER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer3::W](W) writer structure"]
impl crate::Writable for TIMER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER3 to value 0x0a08_0a08"]
impl crate::Resettable for TIMER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a08_0a08
    }
}
