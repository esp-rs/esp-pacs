#[doc = "Register `REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Register `REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER` writer"]
pub type W = crate::W<REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Field `ESC` reader - Enable Slot Comparison When set, this bit enables the checking of the slot numbers, programmed in the transmit descriptor, with the current reference given in Bits \\[19:16\\] The DMA fetches the data from the corresponding buffer only when the slot number is equal to the reference slot number or is ahead of the reference slot number by one slot When reset, this bit disables the checking of the slot numbers The DMA fetches the data immediately after the descriptor is processed"]
pub type ESC_R = crate::BitReader;
#[doc = "Field `ESC` writer - Enable Slot Comparison When set, this bit enables the checking of the slot numbers, programmed in the transmit descriptor, with the current reference given in Bits \\[19:16\\] The DMA fetches the data from the corresponding buffer only when the slot number is equal to the reference slot number or is ahead of the reference slot number by one slot When reset, this bit disables the checking of the slot numbers The DMA fetches the data immediately after the descriptor is processed"]
pub type ESC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASC` reader - Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number _SLOTNUM_ programmed in the transmit descriptor is: equal to the reference slot number given in Bits \\[19:16\\] or ahead of the reference slot number by up to two slots This bit is applicable only when Bit 0 _ESC_ is set"]
pub type ASC_R = crate::BitReader;
#[doc = "Field `ASC` writer - Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number _SLOTNUM_ programmed in the transmit descriptor is: equal to the reference slot number given in Bits \\[19:16\\] or ahead of the reference slot number by up to two slots This bit is applicable only when Bit 0 _ESC_ is set"]
pub type ASC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSN` reader - Reference Slot Number This field gives the current value of the reference slot number in DMA used for comparison checking"]
pub type RSN_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers, programmed in the transmit descriptor, with the current reference given in Bits \\[19:16\\] The DMA fetches the data from the corresponding buffer only when the slot number is equal to the reference slot number or is ahead of the reference slot number by one slot When reset, this bit disables the checking of the slot numbers The DMA fetches the data immediately after the descriptor is processed"]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number _SLOTNUM_ programmed in the transmit descriptor is: equal to the reference slot number given in Bits \\[19:16\\] or ahead of the reference slot number by up to two slots This bit is applicable only when Bit 0 _ESC_ is set"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reference Slot Number This field gives the current value of the reference slot number in DMA used for comparison checking"]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER")
            .field("esc", &self.esc())
            .field("asc", &self.asc())
            .field("rsn", &self.rsn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers, programmed in the transmit descriptor, with the current reference given in Bits \\[19:16\\] The DMA fetches the data from the corresponding buffer only when the slot number is equal to the reference slot number or is ahead of the reference slot number by one slot When reset, this bit disables the checking of the slot numbers The DMA fetches the data immediately after the descriptor is processed"]
    #[inline(always)]
    pub fn esc(
        &mut self,
    ) -> ESC_W<'_, REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC> {
        ESC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number _SLOTNUM_ programmed in the transmit descriptor is: equal to the reference slot number given in Bits \\[19:16\\] or ahead of the reference slot number by up to two slots This bit is applicable only when Bit 0 _ESC_ is set"]
    #[inline(always)]
    pub fn asc(
        &mut self,
    ) -> ASC_W<'_, REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC> {
        ASC_W::new(self, 1)
    }
}
#[doc = "Contains the control bits for slot function and its status for Channel 1 transmit path\n\nYou can [`read`](crate::Reg::read) this register and get [`register76_channel1slotfunctioncontrolandstatusregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register76_channel1slotfunctioncontrolandstatusregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register76_channel1slotfunctioncontrolandstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register76_channel1slotfunctioncontrolandstatusregister::W`](W) writer structure"]
impl crate::Writable for REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC {}
