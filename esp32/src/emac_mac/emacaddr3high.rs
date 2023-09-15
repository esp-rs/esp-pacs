#[doc = "Register `EMACADDR3HIGH` reader"]
pub type R = crate::R<EMACADDR3HIGH_SPEC>;
#[doc = "Register `EMACADDR3HIGH` writer"]
pub type W = crate::W<EMACADDR3HIGH_SPEC>;
#[doc = "Field `MAC_ADDRESS3_HI` reader - This field contains the upper 16 bits Bits\\[47:32\\] of the fourth 6-byte MAC address."]
pub type MAC_ADDRESS3_HI_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_ADDRESS3_HI` writer - This field contains the upper 16 bits Bits\\[47:32\\] of the fourth 6-byte MAC address."]
pub type MAC_ADDRESS3_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MASK_BYTE_CONTROL3` reader - These bits are mask control bits for comparison of each of the EMACADDR3 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR3 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR3 High \\[15:8\\]. Bit\\[28\\]: EMACADDR3 High \\[7:0\\]. Bit\\[27\\]: EMACADDR3 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR3 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL3_R = crate::FieldReader;
#[doc = "Field `MASK_BYTE_CONTROL3` writer - These bits are mask control bits for comparison of each of the EMACADDR3 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR3 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR3 High \\[15:8\\]. Bit\\[28\\]: EMACADDR3 High \\[7:0\\]. Bit\\[27\\]: EMACADDR3 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR3 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
pub type MASK_BYTE_CONTROL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SOURCE_ADDRESS3` reader - When this bit is set the EMACADDR3\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR3\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS3_R = crate::BitReader;
#[doc = "Field `SOURCE_ADDRESS3` writer - When this bit is set the EMACADDR3\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR3\\[47:0\\] is used to compare with the DA fields of the received frame."]
pub type SOURCE_ADDRESS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADDRESS_ENABLE3` reader - When this bit is set the address filter module uses the fourth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE3_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE3` writer - When this bit is set the address filter module uses the fourth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
pub type ADDRESS_ENABLE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the fourth 6-byte MAC address."]
    #[inline(always)]
    pub fn mac_address3_hi(&self) -> MAC_ADDRESS3_HI_R {
        MAC_ADDRESS3_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR3 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR3 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR3 High \\[15:8\\]. Bit\\[28\\]: EMACADDR3 High \\[7:0\\]. Bit\\[27\\]: EMACADDR3 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR3 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    pub fn mask_byte_control3(&self) -> MASK_BYTE_CONTROL3_R {
        MASK_BYTE_CONTROL3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR3\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR3\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    pub fn source_address3(&self) -> SOURCE_ADDRESS3_R {
        SOURCE_ADDRESS3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the fourth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    pub fn address_enable3(&self) -> ADDRESS_ENABLE3_R {
        ADDRESS_ENABLE3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACADDR3HIGH")
            .field(
                "mac_address3_hi",
                &format_args!("{}", self.mac_address3_hi().bits()),
            )
            .field(
                "mask_byte_control3",
                &format_args!("{}", self.mask_byte_control3().bits()),
            )
            .field(
                "source_address3",
                &format_args!("{}", self.source_address3().bit()),
            )
            .field(
                "address_enable3",
                &format_args!("{}", self.address_enable3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACADDR3HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits Bits\\[47:32\\] of the fourth 6-byte MAC address."]
    #[inline(always)]
    #[must_use]
    pub fn mac_address3_hi(&mut self) -> MAC_ADDRESS3_HI_W<EMACADDR3HIGH_SPEC, 0> {
        MAC_ADDRESS3_HI_W::new(self)
    }
    #[doc = "Bits 24:29 - These bits are mask control bits for comparison of each of the EMACADDR3 bytes. When set high the MAC does not compare the corresponding byte of received DA or SA with the contents of EMACADDR3 registers. Each bit controls the masking of the bytes as follows: Bit\\[29\\]: EMACADDR3 High \\[15:8\\]. Bit\\[28\\]: EMACADDR3 High \\[7:0\\]. Bit\\[27\\]: EMACADDR3 Low \\[31:24\\]. Bit\\[24\\]: EMACADDR3 Low \\[7:0\\].You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
    #[inline(always)]
    #[must_use]
    pub fn mask_byte_control3(&mut self) -> MASK_BYTE_CONTROL3_W<EMACADDR3HIGH_SPEC, 24> {
        MASK_BYTE_CONTROL3_W::new(self)
    }
    #[doc = "Bit 30 - When this bit is set the EMACADDR3\\[47:0\\] is used to compare with the SA fields of the received frame. When this bit is reset the EMACADDR3\\[47:0\\] is used to compare with the DA fields of the received frame."]
    #[inline(always)]
    #[must_use]
    pub fn source_address3(&mut self) -> SOURCE_ADDRESS3_W<EMACADDR3HIGH_SPEC, 30> {
        SOURCE_ADDRESS3_W::new(self)
    }
    #[doc = "Bit 31 - When this bit is set the address filter module uses the fourth MAC address for perfect filtering. When this bit is reset the address filter module ignores the address for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn address_enable3(&mut self) -> ADDRESS_ENABLE3_W<EMACADDR3HIGH_SPEC, 31> {
        ADDRESS_ENABLE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Upper 16 bits of the fourth 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr3high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr3high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR3HIGH_SPEC;
impl crate::RegisterSpec for EMACADDR3HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr3high::R`](R) reader structure"]
impl crate::Readable for EMACADDR3HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr3high::W`](W) writer structure"]
impl crate::Writable for EMACADDR3HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACADDR3HIGH to value 0"]
impl crate::Resettable for EMACADDR3HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
