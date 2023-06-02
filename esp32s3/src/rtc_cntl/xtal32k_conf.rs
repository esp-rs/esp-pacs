#[doc = "Register `XTAL32K_CONF` reader"]
pub struct R(crate::R<XTAL32K_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CONF` writer"]
pub struct W(crate::W<XTAL32K_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CONF_SPEC>;
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
impl From<crate::W<XTAL32K_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_RETURN_WAIT` reader - cycles to wait to return noral xtal 32k"]
pub type XTAL32K_RETURN_WAIT_R = crate::FieldReader;
#[doc = "Field `XTAL32K_RETURN_WAIT` writer - cycles to wait to return noral xtal 32k"]
pub type XTAL32K_RETURN_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_CONF_SPEC, 4, O>;
#[doc = "Field `XTAL32K_RESTART_WAIT` reader - cycles to wait to repower on xtal 32k"]
pub type XTAL32K_RESTART_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XTAL32K_RESTART_WAIT` writer - cycles to wait to repower on xtal 32k"]
pub type XTAL32K_RESTART_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, XTAL32K_CONF_SPEC, 16, O, u16, u16>;
#[doc = "Field `XTAL32K_WDT_TIMEOUT` reader - If no clock detected for this amount of time 32k is regarded as dead"]
pub type XTAL32K_WDT_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `XTAL32K_WDT_TIMEOUT` writer - If no clock detected for this amount of time 32k is regarded as dead"]
pub type XTAL32K_WDT_TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_CONF_SPEC, 8, O>;
#[doc = "Field `XTAL32K_STABLE_THRES` reader - if restarted xtal32k period is smaller than this, it is regarded as stable"]
pub type XTAL32K_STABLE_THRES_R = crate::FieldReader;
#[doc = "Field `XTAL32K_STABLE_THRES` writer - if restarted xtal32k period is smaller than this, it is regarded as stable"]
pub type XTAL32K_STABLE_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, XTAL32K_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - cycles to wait to return noral xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_return_wait(&self) -> XTAL32K_RETURN_WAIT_R {
        XTAL32K_RETURN_WAIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - cycles to wait to repower on xtal 32k"]
    #[inline(always)]
    pub fn xtal32k_restart_wait(&self) -> XTAL32K_RESTART_WAIT_R {
        XTAL32K_RESTART_WAIT_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:27 - If no clock detected for this amount of time 32k is regarded as dead"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&self) -> XTAL32K_WDT_TIMEOUT_R {
        XTAL32K_WDT_TIMEOUT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this, it is regarded as stable"]
    #[inline(always)]
    pub fn xtal32k_stable_thres(&self) -> XTAL32K_STABLE_THRES_R {
        XTAL32K_STABLE_THRES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K_CONF")
            .field(
                "xtal32k_return_wait",
                &format_args!("{}", self.xtal32k_return_wait().bits()),
            )
            .field(
                "xtal32k_restart_wait",
                &format_args!("{}", self.xtal32k_restart_wait().bits()),
            )
            .field(
                "xtal32k_wdt_timeout",
                &format_args!("{}", self.xtal32k_wdt_timeout().bits()),
            )
            .field(
                "xtal32k_stable_thres",
                &format_args!("{}", self.xtal32k_stable_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL32K_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - cycles to wait to return noral xtal 32k"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_return_wait(&mut self) -> XTAL32K_RETURN_WAIT_W<0> {
        XTAL32K_RETURN_WAIT_W::new(self)
    }
    #[doc = "Bits 4:19 - cycles to wait to repower on xtal 32k"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_restart_wait(&mut self) -> XTAL32K_RESTART_WAIT_W<4> {
        XTAL32K_RESTART_WAIT_W::new(self)
    }
    #[doc = "Bits 20:27 - If no clock detected for this amount of time 32k is regarded as dead"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_wdt_timeout(&mut self) -> XTAL32K_WDT_TIMEOUT_W<20> {
        XTAL32K_WDT_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this, it is regarded as stable"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_stable_thres(&mut self) -> XTAL32K_STABLE_THRES_W<28> {
        XTAL32K_STABLE_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure xtal32k\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_conf](index.html) module"]
pub struct XTAL32K_CONF_SPEC;
impl crate::RegisterSpec for XTAL32K_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_conf::R](R) reader structure"]
impl crate::Readable for XTAL32K_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_conf::W](W) writer structure"]
impl crate::Writable for XTAL32K_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL32K_CONF to value 0x0ff0_0000"]
impl crate::Resettable for XTAL32K_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ff0_0000;
}
