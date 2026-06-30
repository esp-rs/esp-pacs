#[doc = "Register `REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Register `REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER` writer"]
pub type W = crate::W<REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC>;
#[doc = "Field `TDESLA` reader - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type TDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
pub type TDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list The LSB bits _1:0, 2:0, 3:0_ for 32bit, 64bit, or 128bit bus width are ignored and are internally taken as allzero by the DMA Therefore, these LSB bits are readonly _RO_"]
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W<'_, REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC> {
        TDESLA_W::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register4_transmitdescriptorlistaddressregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register4_transmitdescriptorlistaddressregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register4_transmitdescriptorlistaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register4_transmitdescriptorlistaddressregister::W`](W) writer structure"]
impl crate::Writable for REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC {}
