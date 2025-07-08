#[doc = "Register `SLEEP_CONF0` reader"]
pub type R = crate::R<SLEEP_CONF0_SPEC>;
#[doc = "Register `SLEEP_CONF0` writer"]
pub type W = crate::W<SLEEP_CONF0_SPEC>;
#[doc = "Field `WK_CHAR1` reader - Configures wakeup character 1."]
pub type WK_CHAR1_R = crate::FieldReader;
#[doc = "Field `WK_CHAR1` writer - Configures wakeup character 1."]
pub type WK_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR2` reader - Configures wakeup character 2."]
pub type WK_CHAR2_R = crate::FieldReader;
#[doc = "Field `WK_CHAR2` writer - Configures wakeup character 2."]
pub type WK_CHAR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR3` reader - Configures wakeup character 3."]
pub type WK_CHAR3_R = crate::FieldReader;
#[doc = "Field `WK_CHAR3` writer - Configures wakeup character 3."]
pub type WK_CHAR3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR4` reader - Configures wakeup character 4."]
pub type WK_CHAR4_R = crate::FieldReader;
#[doc = "Field `WK_CHAR4` writer - Configures wakeup character 4."]
pub type WK_CHAR4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures wakeup character 1."]
    #[inline(always)]
    pub fn wk_char1(&self) -> WK_CHAR1_R {
        WK_CHAR1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures wakeup character 2."]
    #[inline(always)]
    pub fn wk_char2(&self) -> WK_CHAR2_R {
        WK_CHAR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures wakeup character 3."]
    #[inline(always)]
    pub fn wk_char3(&self) -> WK_CHAR3_R {
        WK_CHAR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configures wakeup character 4."]
    #[inline(always)]
    pub fn wk_char4(&self) -> WK_CHAR4_R {
        WK_CHAR4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF0")
            .field("wk_char1", &self.wk_char1())
            .field("wk_char2", &self.wk_char2())
            .field("wk_char3", &self.wk_char3())
            .field("wk_char4", &self.wk_char4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures wakeup character 1."]
    #[inline(always)]
    pub fn wk_char1(&mut self) -> WK_CHAR1_W<SLEEP_CONF0_SPEC> {
        WK_CHAR1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures wakeup character 2."]
    #[inline(always)]
    pub fn wk_char2(&mut self) -> WK_CHAR2_W<SLEEP_CONF0_SPEC> {
        WK_CHAR2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures wakeup character 3."]
    #[inline(always)]
    pub fn wk_char3(&mut self) -> WK_CHAR3_W<SLEEP_CONF0_SPEC> {
        WK_CHAR3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configures wakeup character 4."]
    #[inline(always)]
    pub fn wk_char4(&mut self) -> WK_CHAR4_W<SLEEP_CONF0_SPEC> {
        WK_CHAR4_W::new(self, 24)
    }
}
#[doc = "UART sleep configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF0_SPEC;
impl crate::RegisterSpec for SLEEP_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf0::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf0::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF0 to value 0"]
impl crate::Resettable for SLEEP_CONF0_SPEC {}
