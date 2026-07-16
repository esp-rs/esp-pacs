#[doc = "Register `REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Register `REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER` writer"]
pub type W = crate::W<REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Field `CH2_TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH2_TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH2_TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch2_tdesla(&self) -> CH2_TDESLA_R {
        CH2_TDESLA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER")
            .field("ch2_tdesla", &self.ch2_tdesla())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch2_tdesla(
        &mut self,
    ) -> CH2_TDESLA_W<'_, REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC> {
        CH2_TDESLA_W::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Transmit Descriptor List\n\nYou can [`read`](crate::Reg::read) this register and get [`register132_channel2transmitdescriptorlistaddressregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register132_channel2transmitdescriptorlistaddressregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register132_channel2transmitdescriptorlistaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register132_channel2transmitdescriptorlistaddressregister::W`](W) writer structure"]
impl crate::Writable for REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
