#[doc = "Register `RX_FILT` reader"]
pub type R = crate::R<RX_FILT_SPEC>;
#[doc = "Register `RX_FILT` writer"]
pub type W = crate::W<RX_FILT_SPEC>;
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower than this value, the pulse is ignored."]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower than this value, the pulse is ignored."]
pub type GLITCH_FILT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GLITCH_FILT_EN` reader - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_R = crate::BitReader;
#[doc = "Field `GLITCH_FILT_EN` writer - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value, the pulse is ignored."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set this bit to enable Rx signal filter."]
    #[inline(always)]
    pub fn glitch_filt_en(&self) -> GLITCH_FILT_EN_R {
        GLITCH_FILT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_FILT")
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .field(
                "glitch_filt_en",
                &format_args!("{}", self.glitch_filt_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_FILT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value, the pulse is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<RX_FILT_SPEC> {
        GLITCH_FILT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Set this bit to enable Rx signal filter."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt_en(&mut self) -> GLITCH_FILT_EN_W<RX_FILT_SPEC> {
        GLITCH_FILT_EN_W::new(self, 8)
    }
}
#[doc = "Rx Filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_filt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_filt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FILT_SPEC;
impl crate::RegisterSpec for RX_FILT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_filt::R`](R) reader structure"]
impl crate::Readable for RX_FILT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_filt::W`](W) writer structure"]
impl crate::Writable for RX_FILT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_FILT to value 0x08"]
impl crate::Resettable for RX_FILT_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
