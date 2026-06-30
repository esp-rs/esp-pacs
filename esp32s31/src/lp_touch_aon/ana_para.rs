#[doc = "Register `ANA_PARA` reader"]
pub type R = crate::R<ANA_PARA_SPEC>;
#[doc = "Register `ANA_PARA` writer"]
pub type W = crate::W<ANA_PARA_SPEC>;
#[doc = "Field `TOUCH_TOUCH_BUF_DRV` reader - need_des"]
pub type TOUCH_TOUCH_BUF_DRV_R = crate::FieldReader;
#[doc = "Field `TOUCH_TOUCH_BUF_DRV` writer - need_des"]
pub type TOUCH_TOUCH_BUF_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_TOUCH_EN_CAL` reader - need_des"]
pub type TOUCH_TOUCH_EN_CAL_R = crate::BitReader;
#[doc = "Field `TOUCH_TOUCH_EN_CAL` writer - need_des"]
pub type TOUCH_TOUCH_EN_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_TOUCH_DCAP_CAL` reader - need_des"]
pub type TOUCH_TOUCH_DCAP_CAL_R = crate::FieldReader;
#[doc = "Field `TOUCH_TOUCH_DCAP_CAL` writer - need_des"]
pub type TOUCH_TOUCH_DCAP_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUCH_DREFH` reader - need_des"]
pub type TOUCH_DREFH_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFH` writer - need_des"]
pub type TOUCH_DREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DREFL` reader - need_des"]
pub type TOUCH_DREFL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFL` writer - need_des"]
pub type TOUCH_DREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn touch_touch_buf_drv(&self) -> TOUCH_TOUCH_BUF_DRV_R {
        TOUCH_TOUCH_BUF_DRV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn touch_touch_en_cal(&self) -> TOUCH_TOUCH_EN_CAL_R {
        TOUCH_TOUCH_EN_CAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - need_des"]
    #[inline(always)]
    pub fn touch_touch_dcap_cal(&self) -> TOUCH_TOUCH_DCAP_CAL_R {
        TOUCH_TOUCH_DCAP_CAL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - need_des"]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PARA")
            .field("touch_touch_buf_drv", &self.touch_touch_buf_drv())
            .field("touch_touch_en_cal", &self.touch_touch_en_cal())
            .field("touch_touch_dcap_cal", &self.touch_touch_dcap_cal())
            .field("touch_drefh", &self.touch_drefh())
            .field("touch_drefl", &self.touch_drefl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn touch_touch_buf_drv(&mut self) -> TOUCH_TOUCH_BUF_DRV_W<'_, ANA_PARA_SPEC> {
        TOUCH_TOUCH_BUF_DRV_W::new(self, 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn touch_touch_en_cal(&mut self) -> TOUCH_TOUCH_EN_CAL_W<'_, ANA_PARA_SPEC> {
        TOUCH_TOUCH_EN_CAL_W::new(self, 3)
    }
    #[doc = "Bits 4:10 - need_des"]
    #[inline(always)]
    pub fn touch_touch_dcap_cal(&mut self) -> TOUCH_TOUCH_DCAP_CAL_W<'_, ANA_PARA_SPEC> {
        TOUCH_TOUCH_DCAP_CAL_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W<'_, ANA_PARA_SPEC> {
        TOUCH_DREFH_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - need_des"]
    #[inline(always)]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W<'_, ANA_PARA_SPEC> {
        TOUCH_DREFL_W::new(self, 13)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_PARA_SPEC;
impl crate::RegisterSpec for ANA_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_para::R`](R) reader structure"]
impl crate::Readable for ANA_PARA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_para::W`](W) writer structure"]
impl crate::Writable for ANA_PARA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_PARA to value 0x1800"]
impl crate::Resettable for ANA_PARA_SPEC {
    const RESET_VALUE: u32 = 0x1800;
}
