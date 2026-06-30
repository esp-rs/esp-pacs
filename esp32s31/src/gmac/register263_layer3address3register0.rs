#[doc = "Register `REGISTER263_LAYER3ADDRESS3REGISTER0` reader"]
pub type R = crate::R<REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC>;
#[doc = "Register `REGISTER263_LAYER3ADDRESS3REGISTER0` writer"]
pub type W = crate::W<REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC>;
#[doc = "Field `L3A30` reader - Layer 3 Address 3 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
pub type L3A30_R = crate::FieldReader<u32>;
#[doc = "Field `L3A30` writer - Layer 3 Address 3 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
pub type L3A30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
    #[inline(always)]
    pub fn l3a30(&self) -> L3A30_R {
        L3A30_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER263_LAYER3ADDRESS3REGISTER0")
            .field("l3a30", &self.l3a30())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field When Bit 0 _L3PEN0_ and Bit 2 _L3SAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Source Address field in the IPv6 frames When Bit 0 _L3PEN0_ and Bit 4 _L3DAM0_ are set in Register 256 _Layer 3 and Layer 4 Control Register 0_, this field contains the value to be matched with Bits \\[127:96\\] of the IP Destination Address field in the IPv6 frames When Bit 0 _L3PEN0_ is reset in Register 256 _Layer 3 and Layer 4 Control Register 0_, this register is not used"]
    #[inline(always)]
    pub fn l3a30(&mut self) -> L3A30_W<'_, REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC> {
        L3A30_W::new(self, 0)
    }
}
#[doc = "Layer 3 Address 3 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[127:96\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register263_layer3address3register0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register263_layer3address3register0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC;
impl crate::RegisterSpec for REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register263_layer3address3register0::R`](R) reader structure"]
impl crate::Readable for REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register263_layer3address3register0::W`](W) writer structure"]
impl crate::Writable for REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER263_LAYER3ADDRESS3REGISTER0 to value 0"]
impl crate::Resettable for REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC {}
