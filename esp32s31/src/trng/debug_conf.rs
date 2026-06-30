#[doc = "Register `DEBUG_CONF` reader"]
pub type R = crate::R<DEBUG_CONF_SPEC>;
#[doc = "Register `DEBUG_CONF` writer"]
pub type W = crate::W<DEBUG_CONF_SPEC>;
#[doc = "Field `STARTUP_TEST_START` writer - startup start"]
pub type STARTUP_TEST_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUP_TEST_LIMIT` reader - startup limit"]
pub type STARTUP_TEST_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `STARTUP_TEST_LIMIT` writer - startup limit"]
pub type STARTUP_TEST_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEBUG_DATA` reader - debug data"]
pub type DEBUG_DATA_R = crate::FieldReader;
#[doc = "Field `DEBUG_MODE` reader - 1 for debug mode 0 for nomarl mode"]
pub type DEBUG_MODE_R = crate::BitReader;
#[doc = "Field `DEBUG_MODE` writer - 1 for debug mode 0 for nomarl mode"]
pub type DEBUG_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEALTH_TEST_ERROR_CODE` reader - error code"]
pub type HEALTH_TEST_ERROR_CODE_R = crate::FieldReader;
#[doc = "Field `HEALTH_TEST_END` writer - health test end"]
pub type HEALTH_TEST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEALTH_TEST_BYPASS` reader - health test bypass"]
pub type HEALTH_TEST_BYPASS_R = crate::BitReader;
#[doc = "Field `HEALTH_TEST_BYPASS` writer - health test bypass"]
pub type HEALTH_TEST_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:16 - startup limit"]
    #[inline(always)]
    pub fn startup_test_limit(&self) -> STARTUP_TEST_LIMIT_R {
        STARTUP_TEST_LIMIT_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bits 17:24 - debug data"]
    #[inline(always)]
    pub fn debug_data(&self) -> DEBUG_DATA_R {
        DEBUG_DATA_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - 1 for debug mode 0 for nomarl mode"]
    #[inline(always)]
    pub fn debug_mode(&self) -> DEBUG_MODE_R {
        DEBUG_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - error code"]
    #[inline(always)]
    pub fn health_test_error_code(&self) -> HEALTH_TEST_ERROR_CODE_R {
        HEALTH_TEST_ERROR_CODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 30 - health test bypass"]
    #[inline(always)]
    pub fn health_test_bypass(&self) -> HEALTH_TEST_BYPASS_R {
        HEALTH_TEST_BYPASS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_CONF")
            .field("startup_test_limit", &self.startup_test_limit())
            .field("debug_data", &self.debug_data())
            .field("debug_mode", &self.debug_mode())
            .field("health_test_error_code", &self.health_test_error_code())
            .field("health_test_bypass", &self.health_test_bypass())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - startup start"]
    #[inline(always)]
    pub fn startup_test_start(&mut self) -> STARTUP_TEST_START_W<'_, DEBUG_CONF_SPEC> {
        STARTUP_TEST_START_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - startup limit"]
    #[inline(always)]
    pub fn startup_test_limit(&mut self) -> STARTUP_TEST_LIMIT_W<'_, DEBUG_CONF_SPEC> {
        STARTUP_TEST_LIMIT_W::new(self, 1)
    }
    #[doc = "Bit 25 - 1 for debug mode 0 for nomarl mode"]
    #[inline(always)]
    pub fn debug_mode(&mut self) -> DEBUG_MODE_W<'_, DEBUG_CONF_SPEC> {
        DEBUG_MODE_W::new(self, 25)
    }
    #[doc = "Bit 29 - health test end"]
    #[inline(always)]
    pub fn health_test_end(&mut self) -> HEALTH_TEST_END_W<'_, DEBUG_CONF_SPEC> {
        HEALTH_TEST_END_W::new(self, 29)
    }
    #[doc = "Bit 30 - health test bypass"]
    #[inline(always)]
    pub fn health_test_bypass(&mut self) -> HEALTH_TEST_BYPASS_W<'_, DEBUG_CONF_SPEC> {
        HEALTH_TEST_BYPASS_W::new(self, 30)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CONF_SPEC;
impl crate::RegisterSpec for DEBUG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_conf::R`](R) reader structure"]
impl crate::Readable for DEBUG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_conf::W`](W) writer structure"]
impl crate::Writable for DEBUG_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CONF to value 0x0800"]
impl crate::Resettable for DEBUG_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
