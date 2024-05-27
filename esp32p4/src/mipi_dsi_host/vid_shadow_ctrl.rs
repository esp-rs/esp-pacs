#[doc = "Register `VID_SHADOW_CTRL` reader"]
pub type R = crate::R<VID_SHADOW_CTRL_SPEC>;
#[doc = "Register `VID_SHADOW_CTRL` writer"]
pub type W = crate::W<VID_SHADOW_CTRL_SPEC>;
#[doc = "Field `VID_SHADOW_EN` reader - NA"]
pub type VID_SHADOW_EN_R = crate::BitReader;
#[doc = "Field `VID_SHADOW_EN` writer - NA"]
pub type VID_SHADOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_SHADOW_REQ` reader - NA"]
pub type VID_SHADOW_REQ_R = crate::BitReader;
#[doc = "Field `VID_SHADOW_REQ` writer - NA"]
pub type VID_SHADOW_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_SHADOW_PIN_REQ` reader - NA"]
pub type VID_SHADOW_PIN_REQ_R = crate::BitReader;
#[doc = "Field `VID_SHADOW_PIN_REQ` writer - NA"]
pub type VID_SHADOW_PIN_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn vid_shadow_en(&self) -> VID_SHADOW_EN_R {
        VID_SHADOW_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn vid_shadow_req(&self) -> VID_SHADOW_REQ_R {
        VID_SHADOW_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn vid_shadow_pin_req(&self) -> VID_SHADOW_PIN_REQ_R {
        VID_SHADOW_PIN_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_SHADOW_CTRL")
            .field("vid_shadow_en", &self.vid_shadow_en())
            .field("vid_shadow_req", &self.vid_shadow_req())
            .field("vid_shadow_pin_req", &self.vid_shadow_pin_req())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_shadow_en(&mut self) -> VID_SHADOW_EN_W<VID_SHADOW_CTRL_SPEC> {
        VID_SHADOW_EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_shadow_req(&mut self) -> VID_SHADOW_REQ_W<VID_SHADOW_CTRL_SPEC> {
        VID_SHADOW_REQ_W::new(self, 8)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_shadow_pin_req(&mut self) -> VID_SHADOW_PIN_REQ_W<VID_SHADOW_CTRL_SPEC> {
        VID_SHADOW_PIN_REQ_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_shadow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_shadow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_SHADOW_CTRL_SPEC;
impl crate::RegisterSpec for VID_SHADOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_shadow_ctrl::R`](R) reader structure"]
impl crate::Readable for VID_SHADOW_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_shadow_ctrl::W`](W) writer structure"]
impl crate::Writable for VID_SHADOW_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_SHADOW_CTRL to value 0"]
impl crate::Resettable for VID_SHADOW_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
