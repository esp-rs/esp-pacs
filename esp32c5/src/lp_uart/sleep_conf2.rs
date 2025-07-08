#[doc = "Register `SLEEP_CONF2` reader"]
pub type R = crate::R<SLEEP_CONF2_SPEC>;
#[doc = "Register `SLEEP_CONF2` writer"]
pub type W = crate::W<SLEEP_CONF2_SPEC>;
#[doc = "Field `ACTIVE_THRESHOLD` reader - Configures the number of RXD edge changes to wake up the chip in wakeup mode 0."]
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - Configures the number of RXD edge changes to wake up the chip in wakeup mode 0."]
pub type ACTIVE_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RX_WAKE_UP_THRHD` reader - Configures the number of received data bytes to wake up the chip in wakeup mode 1."]
pub type RX_WAKE_UP_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_WAKE_UP_THRHD` writer - Configures the number of received data bytes to wake up the chip in wakeup mode 1."]
pub type RX_WAKE_UP_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WK_CHAR_NUM` reader - Configures the number of wakeup characters."]
pub type WK_CHAR_NUM_R = crate::FieldReader;
#[doc = "Field `WK_CHAR_NUM` writer - Configures the number of wakeup characters."]
pub type WK_CHAR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WK_CHAR_MASK` reader - Configures whether or not to mask wakeup characters.\\\\ 0: Not mask\\\\ 1: Mask\\\\"]
pub type WK_CHAR_MASK_R = crate::FieldReader;
#[doc = "Field `WK_CHAR_MASK` writer - Configures whether or not to mask wakeup characters.\\\\ 0: Not mask\\\\ 1: Mask\\\\"]
pub type WK_CHAR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WK_MODE_SEL` reader - Configures which wakeup mode to select.\\\\ 0: Mode 0\\\\ 1: Mode 1\\\\ 2: Mode 2\\\\ 3: Mode 3\\\\"]
pub type WK_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `WK_MODE_SEL` writer - Configures which wakeup mode to select.\\\\ 0: Mode 0\\\\ 1: Mode 1\\\\ 2: Mode 2\\\\ 3: Mode 3\\\\"]
pub type WK_MODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Configures the number of RXD edge changes to wake up the chip in wakeup mode 0."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 13:17 - Configures the number of received data bytes to wake up the chip in wakeup mode 1."]
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&self) -> RX_WAKE_UP_THRHD_R {
        RX_WAKE_UP_THRHD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:20 - Configures the number of wakeup characters."]
    #[inline(always)]
    pub fn wk_char_num(&self) -> WK_CHAR_NUM_R {
        WK_CHAR_NUM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:25 - Configures whether or not to mask wakeup characters.\\\\ 0: Not mask\\\\ 1: Mask\\\\"]
    #[inline(always)]
    pub fn wk_char_mask(&self) -> WK_CHAR_MASK_R {
        WK_CHAR_MASK_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - Configures which wakeup mode to select.\\\\ 0: Mode 0\\\\ 1: Mode 1\\\\ 2: Mode 2\\\\ 3: Mode 3\\\\"]
    #[inline(always)]
    pub fn wk_mode_sel(&self) -> WK_MODE_SEL_R {
        WK_MODE_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF2")
            .field("active_threshold", &self.active_threshold())
            .field("rx_wake_up_thrhd", &self.rx_wake_up_thrhd())
            .field("wk_char_num", &self.wk_char_num())
            .field("wk_char_mask", &self.wk_char_mask())
            .field("wk_mode_sel", &self.wk_mode_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Configures the number of RXD edge changes to wake up the chip in wakeup mode 0."]
    #[inline(always)]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W<SLEEP_CONF2_SPEC> {
        ACTIVE_THRESHOLD_W::new(self, 0)
    }
    #[doc = "Bits 13:17 - Configures the number of received data bytes to wake up the chip in wakeup mode 1."]
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&mut self) -> RX_WAKE_UP_THRHD_W<SLEEP_CONF2_SPEC> {
        RX_WAKE_UP_THRHD_W::new(self, 13)
    }
    #[doc = "Bits 18:20 - Configures the number of wakeup characters."]
    #[inline(always)]
    pub fn wk_char_num(&mut self) -> WK_CHAR_NUM_W<SLEEP_CONF2_SPEC> {
        WK_CHAR_NUM_W::new(self, 18)
    }
    #[doc = "Bits 21:25 - Configures whether or not to mask wakeup characters.\\\\ 0: Not mask\\\\ 1: Mask\\\\"]
    #[inline(always)]
    pub fn wk_char_mask(&mut self) -> WK_CHAR_MASK_W<SLEEP_CONF2_SPEC> {
        WK_CHAR_MASK_W::new(self, 21)
    }
    #[doc = "Bits 26:27 - Configures which wakeup mode to select.\\\\ 0: Mode 0\\\\ 1: Mode 1\\\\ 2: Mode 2\\\\ 3: Mode 3\\\\"]
    #[inline(always)]
    pub fn wk_mode_sel(&mut self) -> WK_MODE_SEL_W<SLEEP_CONF2_SPEC> {
        WK_MODE_SEL_W::new(self, 26)
    }
}
#[doc = "LP UART sleep configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF2_SPEC;
impl crate::RegisterSpec for SLEEP_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf2::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf2::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF2 to value 0x0014_20f0"]
impl crate::Resettable for SLEEP_CONF2_SPEC {
    const RESET_VALUE: u32 = 0x0014_20f0;
}
