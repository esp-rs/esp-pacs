#[doc = "Register `BLK0_WDATA2` reader"]
pub struct R(crate::R<BLK0_WDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA2` writer"]
pub struct W(crate::W<BLK0_WDATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA2_SPEC>;
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
impl From<crate::W<BLK0_WDATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_MAC_CRC_HIGH` reader - program for high 24bit WIFI_MAC_Address"]
pub type WIFI_MAC_CRC_HIGH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WIFI_MAC_CRC_HIGH` writer - program for high 24bit WIFI_MAC_Address"]
pub type WIFI_MAC_CRC_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, BLK0_WDATA2_SPEC, 24, O, u32, u32>;
impl R {
    #[doc = "Bits 0:23 - program for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn wifi_mac_crc_high(&self) -> WIFI_MAC_CRC_HIGH_R {
        WIFI_MAC_CRC_HIGH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA2")
            .field(
                "wifi_mac_crc_high",
                &format_args!("{}", self.wifi_mac_crc_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - program for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_crc_high(&mut self) -> WIFI_MAC_CRC_HIGH_W<0> {
        WIFI_MAC_CRC_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata2](index.html) module"]
pub struct BLK0_WDATA2_SPEC;
impl crate::RegisterSpec for BLK0_WDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata2::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata2::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA2 to value 0"]
impl crate::Resettable for BLK0_WDATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
