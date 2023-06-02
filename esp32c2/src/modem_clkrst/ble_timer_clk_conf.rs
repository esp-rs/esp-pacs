#[doc = "Register `BLE_TIMER_CLK_CONF` reader"]
pub struct R(crate::R<BLE_TIMER_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_TIMER_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_TIMER_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_TIMER_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_TIMER_CLK_CONF` writer"]
pub struct W(crate::W<BLE_TIMER_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_TIMER_CLK_CONF_SPEC>;
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
impl From<crate::W<BLE_TIMER_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_TIMER_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLETIMER_USE_XTAL` reader - ."]
pub type BLETIMER_USE_XTAL_R = crate::BitReader;
#[doc = "Field `BLETIMER_USE_XTAL` writer - ."]
pub type BLETIMER_USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, BLE_TIMER_CLK_CONF_SPEC, O>;
#[doc = "Field `BLETIMER_CLK_IS_ACTIVE` reader - ."]
pub type BLETIMER_CLK_IS_ACTIVE_R = crate::BitReader;
#[doc = "Field `BLETIMER_CLK_IS_ACTIVE` writer - ."]
pub type BLETIMER_CLK_IS_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, BLE_TIMER_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn bletimer_use_xtal(&self) -> BLETIMER_USE_XTAL_R {
        BLETIMER_USE_XTAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn bletimer_clk_is_active(&self) -> BLETIMER_CLK_IS_ACTIVE_R {
        BLETIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_TIMER_CLK_CONF")
            .field(
                "bletimer_use_xtal",
                &format_args!("{}", self.bletimer_use_xtal().bit()),
            )
            .field(
                "bletimer_clk_is_active",
                &format_args!("{}", self.bletimer_clk_is_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLE_TIMER_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    #[must_use]
    pub fn bletimer_use_xtal(&mut self) -> BLETIMER_USE_XTAL_W<0> {
        BLETIMER_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn bletimer_clk_is_active(&mut self) -> BLETIMER_CLK_IS_ACTIVE_W<1> {
        BLETIMER_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_timer_clk_conf](index.html) module"]
pub struct BLE_TIMER_CLK_CONF_SPEC;
impl crate::RegisterSpec for BLE_TIMER_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_timer_clk_conf::R](R) reader structure"]
impl crate::Readable for BLE_TIMER_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_timer_clk_conf::W](W) writer structure"]
impl crate::Writable for BLE_TIMER_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLE_TIMER_CLK_CONF to value 0x03"]
impl crate::Resettable for BLE_TIMER_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
