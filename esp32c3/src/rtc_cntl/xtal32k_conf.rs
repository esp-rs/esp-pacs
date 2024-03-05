#[doc = "Register `XTAL32K_CONF` reader"]
pub type R = crate::R<XTAL32K_CONF_SPEC>;
#[doc = "Register `XTAL32K_CONF` writer"]
pub type W = crate::W<XTAL32K_CONF_SPEC>;
#[doc = "Field `XTAL32K_RETURN_WAIT` reader - cycles to wait to return noral xtal 32k"]
pub type XTAL32K_RETURN_WAIT_R = crate::FieldReader;
#[doc = "Field `XTAL32K_RETURN_WAIT` writer - cycles to wait to return noral xtal 32k"]
pub type XTAL32K_RETURN_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTAL32K_RESTART_WAIT` reader - cycles to wait to repower on xtal 32k"]
pub type XTAL32K_RESTART_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `XTAL32K_RESTART_WAIT` writer - cycles to wait to repower on xtal 32k"]
pub type XTAL32K_RESTART_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `XTAL32K_WDT_TIMEOUT` reader - If no clock detected for this amount of time"]
pub type XTAL32K_WDT_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `XTAL32K_WDT_TIMEOUT` writer - If no clock detected for this amount of time"]
pub type XTAL32K_WDT_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XTAL32K_STABLE_THRES` reader - if restarted xtal32k period is smaller than this"]
pub type XTAL32K_STABLE_THRES_R = crate::FieldReader;
#[doc = "Field `XTAL32K_STABLE_THRES` writer - if restarted xtal32k period is smaller than this"]
pub type XTAL32K_STABLE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 20:27 - If no clock detected for this amount of time"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&self) -> XTAL32K_WDT_TIMEOUT_R {
        XTAL32K_WDT_TIMEOUT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this"]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - cycles to wait to return noral xtal 32k"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_return_wait(&mut self) -> XTAL32K_RETURN_WAIT_W<XTAL32K_CONF_SPEC> {
        XTAL32K_RETURN_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - cycles to wait to repower on xtal 32k"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_restart_wait(&mut self) -> XTAL32K_RESTART_WAIT_W<XTAL32K_CONF_SPEC> {
        XTAL32K_RESTART_WAIT_W::new(self, 4)
    }
    #[doc = "Bits 20:27 - If no clock detected for this amount of time"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_wdt_timeout(&mut self) -> XTAL32K_WDT_TIMEOUT_W<XTAL32K_CONF_SPEC> {
        XTAL32K_WDT_TIMEOUT_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - if restarted xtal32k period is smaller than this"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_stable_thres(&mut self) -> XTAL32K_STABLE_THRES_W<XTAL32K_CONF_SPEC> {
        XTAL32K_STABLE_THRES_W::new(self, 28)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL32K_CONF_SPEC;
impl crate::RegisterSpec for XTAL32K_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal32k_conf::R`](R) reader structure"]
impl crate::Readable for XTAL32K_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal32k_conf::W`](W) writer structure"]
impl crate::Writable for XTAL32K_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL32K_CONF to value 0x0ff0_0000"]
impl crate::Resettable for XTAL32K_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0ff0_0000;
}
