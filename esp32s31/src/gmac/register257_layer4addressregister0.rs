#[doc = "Register `REGISTER257_LAYER4ADDRESSREGISTER0` reader"]
pub type R = crate::R<REGISTER257_LAYER4ADDRESSREGISTER0_SPEC>;
#[doc = "Register `REGISTER257_LAYER4ADDRESSREGISTER0` writer"]
pub type W = crate::W<REGISTER257_LAYER4ADDRESSREGISTER0_SPEC>;
#[doc = "Field `L4SP0` reader - Layer 4 Source Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames"]
pub type L4SP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4SP0` writer - Layer 4 Source Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames"]
pub type L4SP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP0` reader - Layer 4 Destination Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames"]
pub type L4DP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4DP0` writer - Layer 4 Destination Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames"]
pub type L4DP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER257_LAYER4ADDRESSREGISTER0")
            .field("l4sp0", &self.l4sp0())
            .field("l4dp0", &self.l4dp0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames"]
    #[inline(always)]
    pub fn l4sp0(&mut self) -> L4SP0_W<'_, REGISTER257_LAYER4ADDRESSREGISTER0_SPEC> {
        L4SP0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When Bit 16 _L4PEN0_ is reset and Bit 20 _L4DPM0_ is set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames When Bit 16 _L4PEN0_ and Bit 20 _L4DPM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames"]
    #[inline(always)]
    pub fn l4dp0(&mut self) -> L4DP0_W<'_, REGISTER257_LAYER4ADDRESSREGISTER0_SPEC> {
        L4DP0_W::new(self, 16)
    }
}
#[doc = "Layer 4 Port number field It contains the 16bit Source and Destination Port numbers of the TCP or UDP frame\n\nYou can [`read`](crate::Reg::read) this register and get [`register257_layer4addressregister0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register257_layer4addressregister0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER257_LAYER4ADDRESSREGISTER0_SPEC;
impl crate::RegisterSpec for REGISTER257_LAYER4ADDRESSREGISTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register257_layer4addressregister0::R`](R) reader structure"]
impl crate::Readable for REGISTER257_LAYER4ADDRESSREGISTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register257_layer4addressregister0::W`](W) writer structure"]
impl crate::Writable for REGISTER257_LAYER4ADDRESSREGISTER0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER257_LAYER4ADDRESSREGISTER0 to value 0"]
impl crate::Resettable for REGISTER257_LAYER4ADDRESSREGISTER0_SPEC {}
