#[doc = "Register `PLCP1` reader"]
pub type R = crate::R<PLCP1_SPEC>;
#[doc = "Register `PLCP1` writer"]
pub type W = crate::W<PLCP1_SPEC>;
#[doc = "Field `LEN` reader - Length of packet (in bytes)"]
pub type LEN_R = crate::FieldReader<u16>;
#[doc = "Field `LEN` writer - Length of packet (in bytes)"]
pub type LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RATE` reader - Packet rate (see wifi_phy_rate_t)"]
pub type RATE_R = crate::FieldReader;
#[doc = "Field `RATE` writer - Packet rate (see wifi_phy_rate_t)"]
pub type RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IS_80211_N` reader - Bit indicating if this is 802.11n"]
pub type IS_80211_N_R = crate::BitReader;
#[doc = "Field `IS_80211_N` writer - Bit indicating if this is 802.11n"]
pub type IS_80211_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNKNOWN_ENABLE` reader - meaning unknown, set to one for TX"]
pub type UNKNOWN_ENABLE_R = crate::FieldReader;
#[doc = "Field `UNKNOWN_ENABLE` writer - meaning unknown, set to one for TX"]
pub type UNKNOWN_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Length of packet (in bytes)"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:16 - Packet rate (see wifi_phy_rate_t)"]
    #[inline(always)]
    pub fn rate(&self) -> RATE_R {
        RATE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - Bit indicating if this is 802.11n"]
    #[inline(always)]
    pub fn is_80211_n(&self) -> IS_80211_N_R {
        IS_80211_N_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - meaning unknown, set to one for TX"]
    #[inline(always)]
    pub fn unknown_enable(&self) -> UNKNOWN_ENABLE_R {
        UNKNOWN_ENABLE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLCP1")
            .field("len", &self.len())
            .field("rate", &self.rate())
            .field("is_80211_n", &self.is_80211_n())
            .field("unknown_enable", &self.unknown_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Length of packet (in bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<PLCP1_SPEC> {
        LEN_W::new(self, 0)
    }
    #[doc = "Bits 12:16 - Packet rate (see wifi_phy_rate_t)"]
    #[inline(always)]
    #[must_use]
    pub fn rate(&mut self) -> RATE_W<PLCP1_SPEC> {
        RATE_W::new(self, 12)
    }
    #[doc = "Bit 25 - Bit indicating if this is 802.11n"]
    #[inline(always)]
    #[must_use]
    pub fn is_80211_n(&mut self) -> IS_80211_N_W<PLCP1_SPEC> {
        IS_80211_N_W::new(self, 25)
    }
    #[doc = "Bits 28:29 - meaning unknown, set to one for TX"]
    #[inline(always)]
    #[must_use]
    pub fn unknown_enable(&mut self) -> UNKNOWN_ENABLE_W<PLCP1_SPEC> {
        UNKNOWN_ENABLE_W::new(self, 28)
    }
}
#[doc = "PLCP1\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLCP1_SPEC;
impl crate::RegisterSpec for PLCP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plcp1::R`](R) reader structure"]
impl crate::Readable for PLCP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plcp1::W`](W) writer structure"]
impl crate::Writable for PLCP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLCP1 to value 0"]
impl crate::Resettable for PLCP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
