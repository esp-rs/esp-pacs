#[doc = "Register `THRES0_CTRL` reader"]
pub type R = crate::R<THRES0_CTRL_SPEC>;
#[doc = "Register `THRES0_CTRL` writer"]
pub type W = crate::W<THRES0_CTRL_SPEC>;
#[doc = "Field `THRES0_CHANNEL` reader - need_des"]
pub type THRES0_CHANNEL_R = crate::FieldReader;
#[doc = "Field `THRES0_CHANNEL` writer - need_des"]
pub type THRES0_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THRES0_EN` reader - need_des"]
pub type THRES0_EN_R = crate::BitReader;
#[doc = "Field `THRES0_EN` writer - need_des"]
pub type THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn thres0_channel(&self) -> THRES0_CHANNEL_R {
        THRES0_CHANNEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES0_CTRL")
            .field("thres0_channel", &self.thres0_channel())
            .field("thres0_en", &self.thres0_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn thres0_channel(&mut self) -> THRES0_CHANNEL_W<'_, THRES0_CTRL_SPEC> {
        THRES0_CHANNEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&mut self) -> THRES0_EN_W<'_, THRES0_CTRL_SPEC> {
        THRES0_EN_W::new(self, 5)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES0_CTRL_SPEC;
impl crate::RegisterSpec for THRES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres0_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres0_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES0_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES0_CTRL to value 0x0d"]
impl crate::Resettable for THRES0_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
