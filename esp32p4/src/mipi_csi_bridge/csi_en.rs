#[doc = "Register `CSI_EN` reader"]
pub type R = crate::R<CSI_EN_SPEC>;
#[doc = "Register `CSI_EN` writer"]
pub type W = crate::W<CSI_EN_SPEC>;
#[doc = "Field `CSI_BRIG_EN` reader - 0: disable csi bridge. 1: enable csi bridge."]
pub type CSI_BRIG_EN_R = crate::BitReader;
#[doc = "Field `CSI_BRIG_EN` writer - 0: disable csi bridge. 1: enable csi bridge."]
pub type CSI_BRIG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: disable csi bridge. 1: enable csi bridge."]
    #[inline(always)]
    pub fn csi_brig_en(&self) -> CSI_BRIG_EN_R {
        CSI_BRIG_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSI_EN")
            .field("csi_brig_en", &self.csi_brig_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 0: disable csi bridge. 1: enable csi bridge."]
    #[inline(always)]
    #[must_use]
    pub fn csi_brig_en(&mut self) -> CSI_BRIG_EN_W<CSI_EN_SPEC> {
        CSI_BRIG_EN_W::new(self, 0)
    }
}
#[doc = "csi bridge enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSI_EN_SPEC;
impl crate::RegisterSpec for CSI_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi_en::R`](R) reader structure"]
impl crate::Readable for CSI_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csi_en::W`](W) writer structure"]
impl crate::Writable for CSI_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI_EN to value 0"]
impl crate::Resettable for CSI_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
