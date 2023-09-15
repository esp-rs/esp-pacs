#[doc = "Register `EMACADDR2HIGH` reader"]
pub type R = crate::R<EMACADDR2HIGH_SPEC>;
#[doc = "Register `EMACADDR2HIGH` writer"]
pub type W = crate::W<EMACADDR2HIGH_SPEC>;
#[doc = "Field `MAC_ADDRESS2_HI` reader - This field contains the upper 16 bits Bits\\[47:32\\] of the third 6-byte MAC address."]
pub type MAC_ADDRESS2_HI_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_ADDRESS2_HI` writer - This field contains the upper 16 bits Bits\\[47:32\\] of the third 6-byte MAC address."]
pub type MAC_ADDRESS2_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MASK_BYTE_CONTROL2` reader - These bits are mask control bits for comparison of each of the EMACADDR2 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR2 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR2 High \\[15:8\\]. Bit\\[28\\]: EMACADDR2 High \\[7:0\\]. Bit\\[27\\]: EMACADDR2 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR2 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL2_R = crate::FieldReader;
#[doc = "Field `MASK_BYTE_CONTROL2` writer - These bits are mask control bits for comparison of each of the EMACADDR2 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR2 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR2 High \\[15:8\\]. Bit\\[28\\]: EMACADDR2 High \\[7:0\\]. Bit\\[27\\]: EMACADDR2 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR2 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SOURCE_ADDRESS2` reader - When this bit is set the EMACADDR2\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR2\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS2_R = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS2` writer - When this bit is set the EMACADDR2\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR2\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDRESS_ENABLE2` reader - When this bit is set the address filter module uses the third MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE2_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE2` writer - When this bit is set the address filter module uses the third MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the third 6-byte MAC address."]
    #[inline(always)]
    pub fn mac_address2_hi(&self) -> MAC_ADDRESS2_HI_R {
        MAC_ADDRESS2_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR2 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR2 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR2 High \\[15:8\\]. Bit\\[28\\]: EMACADDR2 High \\[7:0\\]. Bit\\[27\\]: EMACADDR2 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR2 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control2(&self) -> MASK_BYTE_CONTROL2_R {
        MASK_BYTE_CONTROL2_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR2\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR2\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address2(&self) -> SOURCE_ADDRESS2_R {
        SOURCE_ADDRESS2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the third MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable2(&self) -> ADDRESS_ENABLE2_R {
        ADDRESS_ENABLE2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACADDR2HIGH")
            .field(
                "mac_address2_hi",
                &format_args!("{}", self.mac_address2_hi().bits()),
            )
            .field(
                "mask_byte_control2",
                &format_args!("{}", self.mask_byte_control2().bits()),
            )
            .field(
                "source_address2",
                &format_args!("{}", self.source_address2().bit()),
            )
            .field(
                "address_enable2",
                &format_args!("{}", self.address_enable2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACADDR2HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the third 6-byte MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn mac_address2_hi(&mut self) -> MAC_ADDRESS2_HI_W<EMACADDR2HIGH_SPEC, 0> {
        MAC_ADDRESS2_HI_W::new(self)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR2 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR2 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR2 High \\[15:8\\]. Bit\\[28\\]: EMACADDR2 High \\[7:0\\]. Bit\\[27\\]: EMACADDR2 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR2 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    #[must_use]
    pub fn mask_byte_control2(&mut self) -> MASK_BYTE_CONTROL2_W<EMACADDR2HIGH_SPEC, 24> {
        MASK_BYTE_CONTROL2_W::new(self)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR2\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR2\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    #[must_use]
    pub fn source_address2(&mut self) -> SOURCE_ADDRESS2_W<EMACADDR2HIGH_SPEC, 30> {
        SOURCE_ADDRESS2_W::new(self)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the third MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn address_enable2(&mut self) -> ADDRESS_ENABLE2_W<EMACADDR2HIGH_SPEC, 31> {
        ADDRESS_ENABLE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Upper 16 bits of the third 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr2high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr2high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR2HIGH_SPEC;
impl crate::RegisterSpec for EMACADDR2HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr2high::R`](R) reader structure"]
impl crate::Readable for EMACADDR2HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr2high::W`](W) writer structure"]
impl crate::Writable for EMACADDR2HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACADDR2HIGH to value 0"]
impl crate::Resettable for EMACADDR2HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
