#[doc = "Register `FILTER_CONTROL%s` reader"]
pub type R = crate::R<FILTER_CONTROL_SPEC>;
#[doc = "Register `FILTER_CONTROL%s` writer"]
pub type W = crate::W<FILTER_CONTROL_SPEC>;
#[doc = "Field `BLOCK_UNICAST` reader - Block unicast frames not matching the address filter"]
pub type BLOCK_UNICAST_R = crate::BitReader;
#[doc = "Field `BLOCK_UNICAST` writer - Block unicast frames not matching the address filter"]
pub type BLOCK_UNICAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSSID_CHECK` reader - Check BSSID for filtering"]
pub type BSSID_CHECK_R = crate::BitReader;
#[doc = "Field `BSSID_CHECK` writer - Check BSSID for filtering"]
pub type BSSID_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_MULTICAST` reader - Block multicast frames not matching the address filter"]
pub type BLOCK_MULTICAST_R = crate::BitReader;
#[doc = "Field `BLOCK_MULTICAST` writer - Block multicast frames not matching the address filter"]
pub type BLOCK_MULTICAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_MODE` reader - Receive beacon frames"]
pub type SCAN_MODE_R = crate::BitReader;
#[doc = "Field `SCAN_MODE` writer - Receive beacon frames"]
pub type SCAN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_AND_MGMT_MODE` reader - Receive everything except control frames"]
pub type DATA_AND_MGMT_MODE_R = crate::BitReader;
#[doc = "Field `DATA_AND_MGMT_MODE` writer - Receive everything except control frames"]
pub type DATA_AND_MGMT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Block unicast frames not matching the address filter"]
    #[inline(always)]
    pub fn block_unicast(&self) -> BLOCK_UNICAST_R {
        BLOCK_UNICAST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Check BSSID for filtering"]
    #[inline(always)]
    pub fn bssid_check(&self) -> BSSID_CHECK_R {
        BSSID_CHECK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block multicast frames not matching the address filter"]
    #[inline(always)]
    pub fn block_multicast(&self) -> BLOCK_MULTICAST_R {
        BLOCK_MULTICAST_R::new(((self.bits >> 2) & 1) != 0)
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
        f.debug_struct("FILTER_CONTROL")
            .field("block_unicast", &self.block_unicast())
            .field("bssid_check", &self.bssid_check())
            .field("block_multicast", &self.block_multicast())
            .field("scan_mode", &self.scan_mode())
            .field("data_and_mgmt_mode", &self.data_and_mgmt_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Block unicast frames not matching the address filter"]
    #[inline(always)]
    pub fn block_unicast(&mut self) -> BLOCK_UNICAST_W<'_, FILTER_CONTROL_SPEC> {
        BLOCK_UNICAST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Check BSSID for filtering"]
    #[inline(always)]
    pub fn bssid_check(&mut self) -> BSSID_CHECK_W<'_, FILTER_CONTROL_SPEC> {
        BSSID_CHECK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Block multicast frames not matching the address filter"]
    #[inline(always)]
    pub fn block_multicast(&mut self) -> BLOCK_MULTICAST_W<'_, FILTER_CONTROL_SPEC> {
        BLOCK_MULTICAST_W::new(self, 2)
    }
    #[doc = "Bit 4 - Receive beacon frames"]
    #[inline(always)]
    pub fn scan_mode(&mut self) -> SCAN_MODE_W<'_, FILTER_CONTROL_SPEC> {
        SCAN_MODE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Receive everything except control frames"]
    #[inline(always)]
    pub fn data_and_mgmt_mode(&mut self) -> DATA_AND_MGMT_MODE_W<'_, FILTER_CONTROL_SPEC> {
        DATA_AND_MGMT_MODE_W::new(self, 8)
    }
}
#[doc = "Controls the RX filter for an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CONTROL_SPEC;
impl crate::RegisterSpec for FILTER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_control::R`](R) reader structure"]
impl crate::Readable for FILTER_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_control::W`](W) writer structure"]
impl crate::Writable for FILTER_CONTROL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_CONTROL%s to value 0"]
impl crate::Resettable for FILTER_CONTROL_SPEC {}
