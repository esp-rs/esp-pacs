#[doc = "Register `REGISTER262_LAYER3ADDRESS2REGISTER0` reader"]
pub type R = crate::R<REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC>;
#[doc = "Register `REGISTER262_LAYER3ADDRESS2REGISTER0` writer"]
pub type W = crate::W<REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC>;
#[doc = "Field `L3A20` reader - Layer 3 Address 2 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[95:64\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains value to be matched with Bits \\[95:64\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
pub type L3A20_R = crate::FieldReader<u32>;
#[doc = "Field `L3A20` writer - Layer 3 Address 2 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[95:64\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains value to be matched with Bits \\[95:64\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
pub type L3A20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[95:64\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains value to be matched with Bits \\[95:64\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER262_LAYER3ADDRESS2REGISTER0")
            .field("l3a20", &self.l3a20())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[95:64\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains value to be matched with Bits \\[95:64\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
    #[inline(always)]
    pub fn l3a20(&mut self) -> L3A20_W<'_, REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC> {
        L3A20_W::new(self, 0)
    }
}
#[doc = "Layer 3 Address 2 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[95:64\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register262_layer3address2register0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register262_layer3address2register0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC;
impl crate::RegisterSpec for REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register262_layer3address2register0::R`](R) reader structure"]
impl crate::Readable for REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register262_layer3address2register0::W`](W) writer structure"]
impl crate::Writable for REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER262_LAYER3ADDRESS2REGISTER0 to value 0"]
impl crate::Resettable for REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC {}
