#[doc = "Register `EMACADDR7HIGH` reader"]
pub type R = crate::R<EMACADDR7HIGH_SPEC>;
#[doc = "Register `EMACADDR7HIGH` writer"]
pub type W = crate::W<EMACADDR7HIGH_SPEC>;
#[doc = "Field `MAC_ADDRESS7_HI` reader - This field contains the upper 16 bits Bits\\[47:32\\] of the eighth 6-byte MAC Address."]
pub type MAC_ADDRESS7_HI_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_ADDRESS7_HI` writer - This field contains the upper 16 bits Bits\\[47:32\\] of the eighth 6-byte MAC Address."]
pub type MAC_ADDRESS7_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MASK_BYTE_CONTROL7` reader - These bits are mask control bits for comparison of each of the EMACADDR7 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR7 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR7 High \\[15:8\\]. Bit\\[28\\]: EMACADDR7 High \\[7:0\\]. Bit\\[27\\]: EMACADDR7 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR7 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL7_R = crate::FieldReader;
#[doc = "Field `MASK_BYTE_CONTROL7` writer - These bits are mask control bits for comparison of each of the EMACADDR7 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR7 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR7 High \\[15:8\\]. Bit\\[28\\]: EMACADDR7 High \\[7:0\\]. Bit\\[27\\]: EMACADDR7 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR7 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL7_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SOURCE_ADDRESS7` reader - When this bit is set the EMACADDR7\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR7\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS7_R = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS7` writer - When this bit is set the EMACADDR7\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR7\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS_ENABLE7` reader - When this bit is set the address filter module uses the eighth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE7_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE7` writer - When this bit is set the address filter module uses the eighth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the eighth 6-byte MAC Address."]
    #[inline(always)]
    pub fn mac_address7_hi(&self) -> MAC_ADDRESS7_HI_R {
        MAC_ADDRESS7_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR7 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR7 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR7 High \\[15:8\\]. Bit\\[28\\]: EMACADDR7 High \\[7:0\\]. Bit\\[27\\]: EMACADDR7 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR7 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control7(&self) -> MASK_BYTE_CONTROL7_R {
        MASK_BYTE_CONTROL7_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR7\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR7\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address7(&self) -> SOURCE_ADDRESS7_R {
        SOURCE_ADDRESS7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the eighth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable7(&self) -> ADDRESS_ENABLE7_R {
        ADDRESS_ENABLE7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACADDR7HIGH")
            .field("mac_address7_hi", &self.mac_address7_hi())
            .field("mask_byte_control7", &self.mask_byte_control7())
            .field("source_address7", &self.source_address7())
            .field("address_enable7", &self.address_enable7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the eighth 6-byte MAC Address."]
    #[inline(always)]
    pub fn mac_address7_hi(&mut self) -> MAC_ADDRESS7_HI_W<EMACADDR7HIGH_SPEC> {
        MAC_ADDRESS7_HI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR7 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR7 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR7 High \\[15:8\\]. Bit\\[28\\]: EMACADDR7 High \\[7:0\\]. Bit\\[27\\]: EMACADDR7 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR7 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control7(&mut self) -> MASK_BYTE_CONTROL7_W<EMACADDR7HIGH_SPEC> {
        MASK_BYTE_CONTROL7_W::new(self, 24)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR7\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR7\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address7(&mut self) -> SOURCE_ADDRESS7_W<EMACADDR7HIGH_SPEC> {
        SOURCE_ADDRESS7_W::new(self, 30)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the eighth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable7(&mut self) -> ADDRESS_ENABLE7_W<EMACADDR7HIGH_SPEC> {
        ADDRESS_ENABLE7_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the eighth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr7high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr7high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR7HIGH_SPEC;
impl crate::RegisterSpec for EMACADDR7HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr7high::R`](R) reader structure"]
impl crate::Readable for EMACADDR7HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr7high::W`](W) writer structure"]
impl crate::Writable for EMACADDR7HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR7HIGH to value 0"]
impl crate::Resettable for EMACADDR7HIGH_SPEC {}
