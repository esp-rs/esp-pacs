#[doc = "Register `REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Register `REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER` writer"]
pub type W = crate::W<REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Field `CH1_RDESLA` reader - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list The LSB bits _1:0, 2:0, or 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH1_RDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_RDESLA` writer - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list The LSB bits _1:0, 2:0, or 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type CH1_RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list The LSB bits _1:0, 2:0, or 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch1_rdesla(&self) -> CH1_RDESLA_R {
        CH1_RDESLA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER")
            .field("ch1_rdesla", &self.ch1_rdesla())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list The LSB bits _1:0, 2:0, or 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn ch1_rdesla(
        &mut self,
    ) -> CH1_RDESLA_W<'_, REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC> {
        CH1_RDESLA_W::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register67_channel1receivedescriptorlistaddressregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register67_channel1receivedescriptorlistaddressregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register67_channel1receivedescriptorlistaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register67_channel1receivedescriptorlistaddressregister::W`](W) writer structure"]
impl crate::Writable for REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC {}
