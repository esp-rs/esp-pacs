#[doc = "Register `BLEND_TX_SIZE` reader"]
pub type R = crate::R<BLEND_TX_SIZE_SPEC>;
#[doc = "Register `BLEND_TX_SIZE` writer"]
pub type W = crate::W<BLEND_TX_SIZE_SPEC>;
#[doc = "Field `BLEND_HB` reader - The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
pub type BLEND_HB_R = crate::FieldReader<u16>;
#[doc = "Field `BLEND_HB` writer - The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
pub type BLEND_HB_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BLEND_VB` reader - The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
pub type BLEND_VB_R = crate::FieldReader<u16>;
#[doc = "Field `BLEND_VB` writer - The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
pub type BLEND_VB_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
    #[inline(always)]
    pub fn blend_hb(&self) -> BLEND_HB_R {
        BLEND_HB_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
    #[inline(always)]
    pub fn blend_vb(&self) -> BLEND_VB_R {
        BLEND_VB_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_TX_SIZE")
            .field("blend_hb", &format_args!("{}", self.blend_hb().bits()))
            .field("blend_vb", &format_args!("{}", self.blend_vb().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLEND_TX_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
    #[inline(always)]
    #[must_use]
    pub fn blend_hb(&mut self) -> BLEND_HB_W<BLEND_TX_SIZE_SPEC> {
        BLEND_HB_W::new(self, 0)
    }
    #[doc = "Bits 14:27 - The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel"]
    #[inline(always)]
    #[must_use]
    pub fn blend_vb(&mut self) -> BLEND_VB_W<BLEND_TX_SIZE_SPEC> {
        BLEND_VB_W::new(self, 14)
    }
}
#[doc = "Fix pixel filling mode image size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blend_tx_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_tx_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_TX_SIZE_SPEC;
impl crate::RegisterSpec for BLEND_TX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_tx_size::R`](R) reader structure"]
impl crate::Readable for BLEND_TX_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_tx_size::W`](W) writer structure"]
impl crate::Writable for BLEND_TX_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLEND_TX_SIZE to value 0"]
impl crate::Resettable for BLEND_TX_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
