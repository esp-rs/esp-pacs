#[doc = "Register `EMACADDR6HIGH` reader"]
pub type R = crate::R<EMACADDR6HIGH_SPEC>;
#[doc = "Register `EMACADDR6HIGH` writer"]
pub type W = crate::W<EMACADDR6HIGH_SPEC>;
#[doc = "Field `MAC_ADDRESS6_HI` reader - This field contains the upper 16 bits Bits\\[47:32\\] of the seventh 6-byte MAC Address."]
pub type MAC_ADDRESS6_HI_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_ADDRESS6_HI` writer - This field contains the upper 16 bits Bits\\[47:32\\] of the seventh 6-byte MAC Address."]
pub type MAC_ADDRESS6_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MASK_BYTE_CONTROL6` reader - These bits are mask control bits for comparison of each of the EMACADDR6 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR6 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR6 High \\[15:8\\]. Bit\\[28\\]: EMACADDR6 High \\[7:0\\]. Bit\\[27\\]: EMACADDR6 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR6 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL6_R = crate::FieldReader;
#[doc = "Field `MASK_BYTE_CONTROL6` writer - These bits are mask control bits for comparison of each of the EMACADDR6 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR6 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR6 High \\[15:8\\]. Bit\\[28\\]: EMACADDR6 High \\[7:0\\]. Bit\\[27\\]: EMACADDR6 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR6 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SOURCE_ADDRESS6` reader - When this bit is set the EMACADDR6\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR6\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS6_R = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS6` writer - When this bit is set the EMACADDR6\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR6\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS_ENABLE6` reader - When this bit is set the address filter module uses the seventh MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE6_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE6` writer - When this bit is set the address filter module uses the seventh MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the seventh 6-byte MAC Address."]
    #[inline(always)]
    pub fn mac_address6_hi(&self) -> MAC_ADDRESS6_HI_R {
        MAC_ADDRESS6_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR6 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR6 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR6 High \\[15:8\\]. Bit\\[28\\]: EMACADDR6 High \\[7:0\\]. Bit\\[27\\]: EMACADDR6 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR6 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control6(&self) -> MASK_BYTE_CONTROL6_R {
        MASK_BYTE_CONTROL6_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR6\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR6\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address6(&self) -> SOURCE_ADDRESS6_R {
        SOURCE_ADDRESS6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the seventh MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable6(&self) -> ADDRESS_ENABLE6_R {
        ADDRESS_ENABLE6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACADDR6HIGH")
            .field("mac_address6_hi", &self.mac_address6_hi())
            .field("mask_byte_control6", &self.mask_byte_control6())
            .field("source_address6", &self.source_address6())
            .field("address_enable6", &self.address_enable6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the seventh 6-byte MAC Address."]
    #[inline(always)]
    pub fn mac_address6_hi(&mut self) -> MAC_ADDRESS6_HI_W<EMACADDR6HIGH_SPEC> {
        MAC_ADDRESS6_HI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR6 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR6 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR6 High \\[15:8\\]. Bit\\[28\\]: EMACADDR6 High \\[7:0\\]. Bit\\[27\\]: EMACADDR6 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR6 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control6(&mut self) -> MASK_BYTE_CONTROL6_W<EMACADDR6HIGH_SPEC> {
        MASK_BYTE_CONTROL6_W::new(self, 24)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR6\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR6\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address6(&mut self) -> SOURCE_ADDRESS6_W<EMACADDR6HIGH_SPEC> {
        SOURCE_ADDRESS6_W::new(self, 30)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the seventh MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable6(&mut self) -> ADDRESS_ENABLE6_W<EMACADDR6HIGH_SPEC> {
        ADDRESS_ENABLE6_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the seventh 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr6high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr6high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR6HIGH_SPEC;
impl crate::RegisterSpec for EMACADDR6HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr6high::R`](R) reader structure"]
impl crate::Readable for EMACADDR6HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr6high::W`](W) writer structure"]
impl crate::Writable for EMACADDR6HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR6HIGH to value 0"]
impl crate::Resettable for EMACADDR6HIGH_SPEC {}
