#[doc = "Register `INTERFACE_RX_CONTROL%s` reader"]
pub type R = crate::R<INTERFACE_RX_CONTROL_SPEC>;
#[doc = "Register `INTERFACE_RX_CONTROL%s` writer"]
pub type W = crate::W<INTERFACE_RX_CONTROL_SPEC>;
#[doc = "Field `BSSID_CHECK` reader - Check BSSID for filtering"]
pub type BSSID_CHECK_R = crate::BitReader;
#[doc = "Field `BSSID_CHECK` writer - Check BSSID for filtering"]
pub type BSSID_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_MODE` reader - Receive beacon frames"]
pub type SCAN_MODE_R = crate::BitReader;
#[doc = "Field `SCAN_MODE` writer - Receive beacon frames"]
pub type SCAN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_AND_MGMT_MODE` reader - Receive everything except control frames"]
pub type DATA_AND_MGMT_MODE_R = crate::BitReader;
#[doc = "Field `DATA_AND_MGMT_MODE` writer - Receive everything except control frames"]
pub type DATA_AND_MGMT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Check BSSID for filtering"]
    #[inline(always)]
    pub fn bssid_check(&self) -> BSSID_CHECK_R {
        BSSID_CHECK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive beacon frames"]
    #[inline(always)]
    pub fn scan_mode(&self) -> SCAN_MODE_R {
        SCAN_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive everything except control frames"]
    #[inline(always)]
    pub fn data_and_mgmt_mode(&self) -> DATA_AND_MGMT_MODE_R {
        DATA_AND_MGMT_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERFACE_RX_CONTROL")
            .field("bssid_check", &self.bssid_check())
            .field("scan_mode", &self.scan_mode())
            .field("data_and_mgmt_mode", &self.data_and_mgmt_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Check BSSID for filtering"]
    #[inline(always)]
    pub fn bssid_check(&mut self) -> BSSID_CHECK_W<INTERFACE_RX_CONTROL_SPEC> {
        BSSID_CHECK_W::new(self, 1)
    }
    #[doc = "Bit 4 - Receive beacon frames"]
    #[inline(always)]
    pub fn scan_mode(&mut self) -> SCAN_MODE_W<INTERFACE_RX_CONTROL_SPEC> {
        SCAN_MODE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Receive everything except control frames"]
    #[inline(always)]
    pub fn data_and_mgmt_mode(&mut self) -> DATA_AND_MGMT_MODE_W<INTERFACE_RX_CONTROL_SPEC> {
        DATA_AND_MGMT_MODE_W::new(self, 8)
    }
}
#[doc = "Controls RX for an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`interface_rx_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interface_rx_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERFACE_RX_CONTROL_SPEC;
impl crate::RegisterSpec for INTERFACE_RX_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interface_rx_control::R`](R) reader structure"]
impl crate::Readable for INTERFACE_RX_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interface_rx_control::W`](W) writer structure"]
impl crate::Writable for INTERFACE_RX_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERFACE_RX_CONTROL%s to value 0"]
impl crate::Resettable for INTERFACE_RX_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
