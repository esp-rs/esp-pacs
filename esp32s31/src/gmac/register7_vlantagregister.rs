#[doc = "Register `REGISTER7_VLANTAGREGISTER` reader"]
pub type R = crate::R<REGISTER7_VLANTAGREGISTER_SPEC>;
#[doc = "Register `REGISTER7_VLANTAGREGISTER` writer"]
pub type W = crate::W<REGISTER7_VLANTAGREGISTER_SPEC>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames This field contains the 8021Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames The following list describes the bits of this field: Bits \\[15:13\\]: User Priority Bit 12: Canonical Format Indicator _CFI_ or Drop Eligible Indicator _DEI_ Bits\\[11:0\\]: VLAN tag’s VLAN Identifier _VID_ field When the ETV bit is set, only the VID _Bits\\[11:0\\]_ is used for comparison If VL _VL\\[11:0\\] if ETV is set_ is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames This field contains the 8021Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames The following list describes the bits of this field: Bits \\[15:13\\]: User Priority Bit 12: Canonical Format Indicator _CFI_ or Drop Eligible Indicator _DEI_ Bits\\[11:0\\]: VLAN tag’s VLAN Identifier _VID_ field When the ETV bit is set, only the VID _Bits\\[11:0\\]_ is used for comparison If VL _VL\\[11:0\\] if ETV is set_ is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames"]
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching The frames that do not have matching VLAN Tag are marked as matched When reset, this bit enables the VLAN Tag perfect matching The frames with matched VLAN Tag are marked as matched"]
pub type VTIM_R = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching The frames that do not have matching VLAN Tag are marked as matched When reset, this bit enables the VLAN Tag perfect matching The frames with matched VLAN Tag are marked as matched"]
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable SVLAN When this bit is set, the MAC transmitter and receiver also consider the SVLAN _Type = 0x88A8_ frames as valid VLAN tagged frames"]
pub type ESVL_R = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable SVLAN When this bit is set, the MAC transmitter and receiver also consider the SVLAN _Type = 0x88A8_ frames as valid VLAN tagged frames"]
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 _VLAN Hash Table Register_ A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table When Bit 16 _ETV_ is set, the CRC of the 12bit VLAN Identifier _VID_ is used for comparison whereas when ETV is reset, the CRC of the 16bit VLAN tag is used for comparison When reset, the VLAN Hash Match operation is not performed If the VLAN Hash feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type VTHM_R = crate::BitReader;
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 _VLAN Hash Table Register_ A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table When Bit 16 _ETV_ is set, the CRC of the 12bit VLAN Identifier _VID_ is used for comparison whereas when ETV is reset, the CRC of the 16bit VLAN tag is used for comparison When reset, the VLAN Hash Match operation is not performed If the VLAN Hash feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type VTHM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This field contains the 8021Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames The following list describes the bits of this field: Bits \\[15:13\\]: User Priority Bit 12: Canonical Format Indicator _CFI_ or Drop Eligible Indicator _DEI_ Bits\\[11:0\\]: VLAN tag’s VLAN Identifier _VID_ field When the ETV bit is set, only the VID _Bits\\[11:0\\]_ is used for comparison If VL _VL\\[11:0\\] if ETV is set_ is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching The frames that do not have matching VLAN Tag are marked as matched When reset, this bit enables the VLAN Tag perfect matching The frames with matched VLAN Tag are marked as matched"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable SVLAN When this bit is set, the MAC transmitter and receiver also consider the SVLAN _Type = 0x88A8_ frames as valid VLAN tagged frames"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 _VLAN Hash Table Register_ A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table When Bit 16 _ETV_ is set, the CRC of the 12bit VLAN Identifier _VID_ is used for comparison whereas when ETV is reset, the CRC of the 16bit VLAN tag is used for comparison When reset, the VLAN Hash Match operation is not performed If the VLAN Hash feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER7_VLANTAGREGISTER")
            .field("vl", &self.vl())
            .field("etv", &self.etv())
            .field("vtim", &self.vtim())
            .field("esvl", &self.esvl())
            .field("vthm", &self.vthm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This field contains the 8021Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames The following list describes the bits of this field: Bits \\[15:13\\]: User Priority Bit 12: Canonical Format Indicator _CFI_ or Drop Eligible Indicator _DEI_ Bits\\[11:0\\]: VLAN tag’s VLAN Identifier _VID_ field When the ETV bit is set, only the VID _Bits\\[11:0\\]_ is used for comparison If VL _VL\\[11:0\\] if ETV is set_ is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W<'_, REGISTER7_VLANTAGREGISTER_SPEC> {
        VL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<'_, REGISTER7_VLANTAGREGISTER_SPEC> {
        ETV_W::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching The frames that do not have matching VLAN Tag are marked as matched When reset, this bit enables the VLAN Tag perfect matching The frames with matched VLAN Tag are marked as matched"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W<'_, REGISTER7_VLANTAGREGISTER_SPEC> {
        VTIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable SVLAN When this bit is set, the MAC transmitter and receiver also consider the SVLAN _Type = 0x88A8_ frames as valid VLAN tagged frames"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W<'_, REGISTER7_VLANTAGREGISTER_SPEC> {
        ESVL_W::new(self, 18)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 _VLAN Hash Table Register_ A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table When Bit 16 _ETV_ is set, the CRC of the 12bit VLAN Identifier _VID_ is used for comparison whereas when ETV is reset, the CRC of the 16bit VLAN tag is used for comparison When reset, the VLAN Hash Match operation is not performed If the VLAN Hash feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W<'_, REGISTER7_VLANTAGREGISTER_SPEC> {
        VTHM_W::new(self, 19)
    }
}
#[doc = "Identifies IEEE 8021Q VLAN type frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register7_vlantagregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7_vlantagregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER7_VLANTAGREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER7_VLANTAGREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register7_vlantagregister::R`](R) reader structure"]
impl crate::Readable for REGISTER7_VLANTAGREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register7_vlantagregister::W`](W) writer structure"]
impl crate::Writable for REGISTER7_VLANTAGREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER7_VLANTAGREGISTER to value 0"]
impl crate::Resettable for REGISTER7_VLANTAGREGISTER_SPEC {}
