#[doc = "Register `CROP_CTRL` writer"]
pub type W = crate::W<CROP_CTRL_SPEC>;
#[doc = "Field `CROP_SFT_RST` writer - Write 1 to clear err st"]
pub type CROP_SFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CROP_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear err st"]
    #[inline(always)]
    pub fn crop_sft_rst(&mut self) -> CROP_SFT_RST_W<'_, CROP_CTRL_SPEC> {
        CROP_SFT_RST_W::new(self, 0)
    }
}
#[doc = "isp_crop ctrl register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CROP_CTRL_SPEC;
impl crate::RegisterSpec for CROP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crop_ctrl::W`](W) writer structure"]
impl crate::Writable for CROP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CROP_CTRL to value 0"]
impl crate::Resettable for CROP_CTRL_SPEC {}
