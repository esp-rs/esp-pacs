#[doc = "Register `cali_conf` reader"]
pub type R = crate::R<CALI_CONF_SPEC>;
#[doc = "Register `cali_conf` writer"]
pub type W = crate::W<CALI_CONF_SPEC>;
#[doc = "Field `CALI_RTC_MAX` reader - "]
pub type CALI_RTC_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `CALI_RTC_MAX` writer - "]
pub type CALI_RTC_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CALI_START` reader - "]
pub type CALI_START_R = crate::BitReader;
#[doc = "Field `CALI_START` writer - "]
pub type CALI_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cali_rtc_max(&self) -> CALI_RTC_MAX_R {
        CALI_RTC_MAX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_start(&self) -> CALI_START_R {
        CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cali_conf")
            .field("cali_rtc_max", &self.cali_rtc_max())
            .field("cali_start", &self.cali_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn cali_rtc_max(&mut self) -> CALI_RTC_MAX_W<CALI_CONF_SPEC> {
        CALI_RTC_MAX_W::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cali_start(&mut self) -> CALI_START_W<CALI_CONF_SPEC> {
        CALI_START_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_CONF_SPEC;
impl crate::RegisterSpec for CALI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali_conf::R`](R) reader structure"]
impl crate::Readable for CALI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali_conf::W`](W) writer structure"]
impl crate::Writable for CALI_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cali_conf to value 0"]
impl crate::Resettable for CALI_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
