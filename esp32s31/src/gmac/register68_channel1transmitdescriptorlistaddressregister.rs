#[doc = "Register `REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Register `REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER` writer"]
pub type W = crate::W<REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Field `CH1_TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH1_TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH1_TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch1_tdesla(&self) -> CH1_TDESLA_R {
        CH1_TDESLA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER")
            .field("ch1_tdesla", &self.ch1_tdesla())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch1_tdesla(
        &mut self,
    ) -> CH1_TDESLA_W<'_, REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC> {
        CH1_TDESLA_W::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register68_channel1transmitdescriptorlistaddressregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register68_channel1transmitdescriptorlistaddressregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register68_channel1transmitdescriptorlistaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register68_channel1transmitdescriptorlistaddressregister::W`](W) writer structure"]
impl crate::Writable for REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
