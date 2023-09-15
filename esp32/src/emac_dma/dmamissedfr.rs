#[doc = "Register `DMAMISSEDFR` reader"]
pub type R = crate::R<DMAMISSEDFR_SPEC>;
#[doc = "Register `DMAMISSEDFR` writer"]
pub type W = crate::W<DMAMISSEDFR_SPEC>;
#[doc = "Field `MISSED_FC` reader - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
pub type MISSED_FC_R = crate::FieldReader<u16>;
#[doc = "Field `MISSED_FC` writer - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
pub type MISSED_FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `OVERFLOW_BMFC` reader - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows that is the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type OVERFLOW_BMFC_R = crate::BitReader;
#[doc = "Field `OVERFLOW_BMFC` writer - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows that is the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type OVERFLOW_BMFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFLOW_FC` reader - This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read."]
pub type OVERFLOW_FC_R = crate::FieldReader<u16>;
#[doc = "Field `OVERFLOW_FC` writer - This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read."]
pub type OVERFLOW_FC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `OVERFLOW_BFOC` reader - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows that is the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type OVERFLOW_BFOC_R = crate::BitReader;
#[doc = "Field `OVERFLOW_BFOC` writer - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows that is the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type OVERFLOW_BFOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn missed_fc(&self) -> MISSED_FC_R {
        MISSED_FC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows that is the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn overflow_bmfc(&self) -> OVERFLOW_BMFC_R {
        OVERFLOW_BMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn overflow_fc(&self) -> OVERFLOW_FC_R {
        OVERFLOW_FC_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows that is the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn overflow_bfoc(&self) -> OVERFLOW_BFOC_R {
        OVERFLOW_BFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMISSEDFR")
            .field("missed_fc", &format_args!("{}", self.missed_fc().bits()))
            .field(
                "overflow_bmfc",
                &format_args!("{}", self.overflow_bmfc().bit()),
            )
            .field(
                "overflow_fc",
                &format_args!("{}", self.overflow_fc().bits()),
            )
            .field(
                "overflow_bfoc",
                &format_args!("{}", self.overflow_bfoc().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMAMISSEDFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn missed_fc(&mut self) -> MISSED_FC_W<DMAMISSEDFR_SPEC, 0> {
        MISSED_FC_W::new(self)
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows that is the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_bmfc(&mut self) -> OVERFLOW_BMFC_W<DMAMISSEDFR_SPEC, 16> {
        OVERFLOW_BMFC_W::new(self)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_fc(&mut self) -> OVERFLOW_FC_W<DMAMISSEDFR_SPEC, 17> {
        OVERFLOW_FC_W::new(self)
    }
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows that is the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_bfoc(&mut self) -> OVERFLOW_BFOC_W<DMAMISSEDFR_SPEC, 28> {
        OVERFLOW_BFOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamissedfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamissedfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMISSEDFR_SPEC;
impl crate::RegisterSpec for DMAMISSEDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamissedfr::R`](R) reader structure"]
impl crate::Readable for DMAMISSEDFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmamissedfr::W`](W) writer structure"]
impl crate::Writable for DMAMISSEDFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMISSEDFR to value 0"]
impl crate::Resettable for DMAMISSEDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
