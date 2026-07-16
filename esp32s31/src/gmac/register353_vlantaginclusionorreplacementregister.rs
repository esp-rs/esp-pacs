#[doc = "Register `REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER` reader"]
pub type R = crate::R<REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC>;
#[doc = "Register `REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER` writer"]
pub type W = crate::W<REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC>;
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced The value must only be changed when the transmit lines are inactive or during the initialization phase Bits\\[15:13\\] are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\] are the VLAN tag’s VID field"]
pub type VLT_R = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced The value must only be changed when the transmit lines are inactive or during the initialization phase Bits\\[15:13\\] are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\] are the VLAN tag’s VID field"]
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Frames 2’b00: No VLAN tag deletion, insertion, or replacement 2’b01: VLAN tag deletion The MAC removes the VLAN type _bytes 13 and 14_ and VLAN tag _bytes 15 and 16_ of all transmitted frames with VLAN tags 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value _0x8100/0x88a8_ in bytes 13 and 14 This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLANtype transmitted frames _Bytes 13 and 14 are 0x8100/0x88a8_ Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value"]
pub type VLC_R = crate::FieldReader;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Frames 2’b00: No VLAN tag deletion, insertion, or replacement 2’b01: VLAN tag deletion The MAC removes the VLAN type _bytes 13 and 14_ and VLAN tag _bytes 15 and 16_ of all transmitted frames with VLAN tags 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value _0x8100/0x88a8_ in bytes 13 and 14 This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLANtype transmitted frames _Bytes 13 and 14 are 0x8100/0x88a8_ Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value"]
pub type VLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\] are used for VLAN deletion, insertion, or replacement When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\] are ignored"]
pub type VLP_R = crate::BitReader;
#[doc = "Field `VLP` writer - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\] are used for VLAN deletion, insertion, or replacement When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\] are ignored"]
pub type VLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - CVLAN or SVLAN When this bit is set, SVLAN type _0x88A8_ is inserted or replaced in the 13th and 14th bytes of transmitted frames When this bit is reset, CVLAN type _0x8100_ is inserted or replaced in the transmitted frames"]
pub type CSVL_R = crate::BitReader;
#[doc = "Field `CSVL` writer - CVLAN or SVLAN When this bit is set, SVLAN type _0x88A8_ is inserted or replaced in the 13th and 14th bytes of transmitted frames When this bit is reset, CVLAN type _0x8100_ is inserted or replaced in the transmitted frames"]
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced The value must only be changed when the transmit lines are inactive or during the initialization phase Bits\\[15:13\\] are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\] are the VLAN tag’s VID field"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames 2’b00: No VLAN tag deletion, insertion, or replacement 2’b01: VLAN tag deletion The MAC removes the VLAN type _bytes 13 and 14_ and VLAN tag _bytes 15 and 16_ of all transmitted frames with VLAN tags 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value _0x8100/0x88a8_ in bytes 13 and 14 This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLANtype transmitted frames _Bytes 13 and 14 are 0x8100/0x88a8_ Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\] are used for VLAN deletion, insertion, or replacement When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\] are ignored"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CVLAN or SVLAN When this bit is set, SVLAN type _0x88A8_ is inserted or replaced in the 13th and 14th bytes of transmitted frames When this bit is reset, CVLAN type _0x8100_ is inserted or replaced in the transmitted frames"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced The value must only be changed when the transmit lines are inactive or during the initialization phase Bits\\[15:13\\] are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\] are the VLAN tag’s VID field"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W<'_, REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC> {
        VLT_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames 2’b00: No VLAN tag deletion, insertion, or replacement 2’b01: VLAN tag deletion The MAC removes the VLAN type _bytes 13 and 14_ and VLAN tag _bytes 15 and 16_ of all transmitted frames with VLAN tags 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value _0x8100/0x88a8_ in bytes 13 and 14 This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLANtype transmitted frames _Bytes 13 and 14 are 0x8100/0x88a8_ Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W<'_, REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC> {
        VLC_W::new(self, 16)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the control Bits \\[17:16\\] are used for VLAN deletion, insertion, or replacement When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\] are ignored"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W<'_, REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC> {
        VLP_W::new(self, 18)
    }
    #[doc = "Bit 19 - CVLAN or SVLAN When this bit is set, SVLAN type _0x88A8_ is inserted or replaced in the 13th and 14th bytes of transmitted frames When this bit is reset, CVLAN type _0x8100_ is inserted or replaced in the transmitted frames"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W<'_, REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC> {
        CSVL_W::new(self, 19)
    }
}
#[doc = "This register contains the VLAN tag for insertion into or replacement in the transmit frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register353_vlantaginclusionorreplacementregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register353_vlantaginclusionorreplacementregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register353_vlantaginclusionorreplacementregister::R`](R) reader structure"]
impl crate::Readable for REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register353_vlantaginclusionorreplacementregister::W`](W) writer structure"]
impl crate::Writable for REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER to value 0"]
impl crate::Resettable for REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC {}
