#[doc = "Register `ADDR_HIGH` reader"]
pub type R = crate::R<ADDR_HIGH_SPEC>;
#[doc = "Register `ADDR_HIGH` writer"]
pub type W = crate::W<ADDR_HIGH_SPEC>;
#[doc = "Field `ADDR` reader - Higher two octets of the MAC address"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Higher two octets of the MAC address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WEP_104` reader - Is the cipher WEP104"]
pub type WEP_104_R = crate::BitReader;
#[doc = "Field `WEP_104` writer - Is the cipher WEP104"]
pub type WEP_104_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGORITHM` reader - In use algorithm"]
pub type ALGORITHM_R = crate::FieldReader;
#[doc = "Field `ALGORITHM` writer - In use algorithm"]
pub type ALGORITHM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GROUP_KEY` reader - Is this a group key"]
pub type GROUP_KEY_R = crate::BitReader;
#[doc = "Field `GROUP_KEY` writer - Is this a group key"]
pub type GROUP_KEY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNKNOWN` reader - Meaning unknown set to 1 for all algorithms"]
pub type UNKNOWN_R = crate::BitReader;
#[doc = "Field `UNKNOWN` writer - Meaning unknown set to 1 for all algorithms"]
pub type UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAIRWISE_KEY` reader - Is this a pairwise key"]
pub type PAIRWISE_KEY_R = crate::BitReader;
#[doc = "Field `PAIRWISE_KEY` writer - Is this a pairwise key"]
pub type PAIRWISE_KEY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERFACE_ID` reader - Index of the interface using the key"]
pub type INTERFACE_ID_R = crate::FieldReader;
#[doc = "Field `INTERFACE_ID` writer - Index of the interface using the key"]
pub type INTERFACE_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BITS_256` reader - Key length is 256 bits"]
pub type BITS_256_R = crate::BitReader;
#[doc = "Field `BITS_256` writer - Key length is 256 bits"]
pub type BITS_256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ID` reader - Protocol identifier of the key"]
pub type KEY_ID_R = crate::FieldReader;
#[doc = "Field `KEY_ID` writer - Protocol identifier of the key"]
pub type KEY_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - Higher two octets of the MAC address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Is the cipher WEP104"]
    #[inline(always)]
    pub fn wep_104(&self) -> WEP_104_R {
        WEP_104_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - In use algorithm"]
    #[inline(always)]
    pub fn algorithm(&self) -> ALGORITHM_R {
        ALGORITHM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Is this a group key"]
    #[inline(always)]
    pub fn group_key(&self) -> GROUP_KEY_R {
        GROUP_KEY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Meaning unknown set to 1 for all algorithms"]
    #[inline(always)]
    pub fn unknown(&self) -> UNKNOWN_R {
        UNKNOWN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Is this a pairwise key"]
    #[inline(always)]
    pub fn pairwise_key(&self) -> PAIRWISE_KEY_R {
        PAIRWISE_KEY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Index of the interface using the key"]
    #[inline(always)]
    pub fn interface_id(&self) -> INTERFACE_ID_R {
        INTERFACE_ID_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Key length is 256 bits"]
    #[inline(always)]
    pub fn bits_256(&self) -> BITS_256_R {
        BITS_256_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Protocol identifier of the key"]
    #[inline(always)]
    pub fn key_id(&self) -> KEY_ID_R {
        KEY_ID_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_HIGH")
            .field("addr", &self.addr())
            .field("wep_104", &self.wep_104())
            .field("algorithm", &self.algorithm())
            .field("group_key", &self.group_key())
            .field("unknown", &self.unknown())
            .field("pairwise_key", &self.pairwise_key())
            .field("interface_id", &self.interface_id())
            .field("bits_256", &self.bits_256())
            .field("key_id", &self.key_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Higher two octets of the MAC address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<ADDR_HIGH_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 16 - Is the cipher WEP104"]
    #[inline(always)]
    pub fn wep_104(&mut self) -> WEP_104_W<ADDR_HIGH_SPEC> {
        WEP_104_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - In use algorithm"]
    #[inline(always)]
    pub fn algorithm(&mut self) -> ALGORITHM_W<ADDR_HIGH_SPEC> {
        ALGORITHM_W::new(self, 18)
    }
    #[doc = "Bit 21 - Is this a group key"]
    #[inline(always)]
    pub fn group_key(&mut self) -> GROUP_KEY_W<ADDR_HIGH_SPEC> {
        GROUP_KEY_W::new(self, 21)
    }
    #[doc = "Bit 22 - Meaning unknown set to 1 for all algorithms"]
    #[inline(always)]
    pub fn unknown(&mut self) -> UNKNOWN_W<ADDR_HIGH_SPEC> {
        UNKNOWN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Is this a pairwise key"]
    #[inline(always)]
    pub fn pairwise_key(&mut self) -> PAIRWISE_KEY_W<ADDR_HIGH_SPEC> {
        PAIRWISE_KEY_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Index of the interface using the key"]
    #[inline(always)]
    pub fn interface_id(&mut self) -> INTERFACE_ID_W<ADDR_HIGH_SPEC> {
        INTERFACE_ID_W::new(self, 24)
    }
    #[doc = "Bit 26 - Key length is 256 bits"]
    #[inline(always)]
    pub fn bits_256(&mut self) -> BITS_256_W<ADDR_HIGH_SPEC> {
        BITS_256_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Protocol identifier of the key"]
    #[inline(always)]
    pub fn key_id(&mut self) -> KEY_ID_W<ADDR_HIGH_SPEC> {
        KEY_ID_W::new(self, 30)
    }
}
#[doc = "Higher two octets of the MAC address and config bits\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_HIGH_SPEC;
impl crate::RegisterSpec for ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_high::R`](R) reader structure"]
impl crate::Readable for ADDR_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_high::W`](W) writer structure"]
impl crate::Writable for ADDR_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR_HIGH to value 0"]
impl crate::Resettable for ADDR_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
