#[doc = "Register `SRM_SCAL_ROTATE` reader"]
pub type R = crate::R<SRM_SCAL_ROTATE_SPEC>;
#[doc = "Register `SRM_SCAL_ROTATE` writer"]
pub type W = crate::W<SRM_SCAL_ROTATE_SPEC>;
#[doc = "Field `SRM_SCAL_X_INT` reader - The integrated part of scaling coefficient in X direction."]
pub type SRM_SCAL_X_INT_R = crate::FieldReader;
#[doc = "Field `SRM_SCAL_X_INT` writer - The integrated part of scaling coefficient in X direction."]
pub type SRM_SCAL_X_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRM_SCAL_X_FRAG` reader - The fragment part of scaling coefficient in X direction."]
pub type SRM_SCAL_X_FRAG_R = crate::FieldReader;
#[doc = "Field `SRM_SCAL_X_FRAG` writer - The fragment part of scaling coefficient in X direction."]
pub type SRM_SCAL_X_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRM_SCAL_Y_INT` reader - The integrated part of scaling coefficient in Y direction."]
pub type SRM_SCAL_Y_INT_R = crate::FieldReader;
#[doc = "Field `SRM_SCAL_Y_INT` writer - The integrated part of scaling coefficient in Y direction."]
pub type SRM_SCAL_Y_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRM_SCAL_Y_FRAG` reader - The fragment part of scaling coefficient in Y direction."]
pub type SRM_SCAL_Y_FRAG_R = crate::FieldReader;
#[doc = "Field `SRM_SCAL_Y_FRAG` writer - The fragment part of scaling coefficient in Y direction."]
pub type SRM_SCAL_Y_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRM_ROTATE_ANGLE` reader - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree."]
pub type SRM_ROTATE_ANGLE_R = crate::FieldReader;
#[doc = "Field `SRM_ROTATE_ANGLE` writer - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree."]
pub type SRM_ROTATE_ANGLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAL_ROTATE_RST` reader - Write 1 then write 0 to this bit to reset scaling and rotating engine."]
pub type SCAL_ROTATE_RST_R = crate::BitReader;
#[doc = "Field `SCAL_ROTATE_RST` writer - Write 1 then write 0 to this bit to reset scaling and rotating engine."]
pub type SCAL_ROTATE_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAL_ROTATE_START` writer - Write 1 to enable scaling and rotating engine after parameter is configured."]
pub type SCAL_ROTATE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_MIRROR_X` reader - Image mirror in X direction. 0: disable, 1: enable"]
pub type SRM_MIRROR_X_R = crate::BitReader;
#[doc = "Field `SRM_MIRROR_X` writer - Image mirror in X direction. 0: disable, 1: enable"]
pub type SRM_MIRROR_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_MIRROR_Y` reader - Image mirror in Y direction. 0: disable, 1: enable"]
pub type SRM_MIRROR_Y_R = crate::BitReader;
#[doc = "Field `SRM_MIRROR_Y` writer - Image mirror in Y direction. 0: disable, 1: enable"]
pub type SRM_MIRROR_Y_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The integrated part of scaling coefficient in X direction."]
    #[inline(always)]
    pub fn srm_scal_x_int(&self) -> SRM_SCAL_X_INT_R {
        SRM_SCAL_X_INT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - The fragment part of scaling coefficient in X direction."]
    #[inline(always)]
    pub fn srm_scal_x_frag(&self) -> SRM_SCAL_X_FRAG_R {
        SRM_SCAL_X_FRAG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:19 - The integrated part of scaling coefficient in Y direction."]
    #[inline(always)]
    pub fn srm_scal_y_int(&self) -> SRM_SCAL_Y_INT_R {
        SRM_SCAL_Y_INT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - The fragment part of scaling coefficient in Y direction."]
    #[inline(always)]
    pub fn srm_scal_y_frag(&self) -> SRM_SCAL_Y_FRAG_R {
        SRM_SCAL_Y_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree."]
    #[inline(always)]
    pub fn srm_rotate_angle(&self) -> SRM_ROTATE_ANGLE_R {
        SRM_ROTATE_ANGLE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset scaling and rotating engine."]
    #[inline(always)]
    pub fn scal_rotate_rst(&self) -> SCAL_ROTATE_RST_R {
        SCAL_ROTATE_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Image mirror in X direction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn srm_mirror_x(&self) -> SRM_MIRROR_X_R {
        SRM_MIRROR_X_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Image mirror in Y direction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn srm_mirror_y(&self) -> SRM_MIRROR_Y_R {
        SRM_MIRROR_Y_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRM_SCAL_ROTATE")
            .field("srm_scal_x_int", &self.srm_scal_x_int())
            .field("srm_scal_x_frag", &self.srm_scal_x_frag())
            .field("srm_scal_y_int", &self.srm_scal_y_int())
            .field("srm_scal_y_frag", &self.srm_scal_y_frag())
            .field("srm_rotate_angle", &self.srm_rotate_angle())
            .field("scal_rotate_rst", &self.scal_rotate_rst())
            .field("srm_mirror_x", &self.srm_mirror_x())
            .field("srm_mirror_y", &self.srm_mirror_y())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The integrated part of scaling coefficient in X direction."]
    #[inline(always)]
    pub fn srm_scal_x_int(&mut self) -> SRM_SCAL_X_INT_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_SCAL_X_INT_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - The fragment part of scaling coefficient in X direction."]
    #[inline(always)]
    pub fn srm_scal_x_frag(&mut self) -> SRM_SCAL_X_FRAG_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_SCAL_X_FRAG_W::new(self, 8)
    }
    #[doc = "Bits 12:19 - The integrated part of scaling coefficient in Y direction."]
    #[inline(always)]
    pub fn srm_scal_y_int(&mut self) -> SRM_SCAL_Y_INT_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_SCAL_Y_INT_W::new(self, 12)
    }
    #[doc = "Bits 20:23 - The fragment part of scaling coefficient in Y direction."]
    #[inline(always)]
    pub fn srm_scal_y_frag(&mut self) -> SRM_SCAL_Y_FRAG_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_SCAL_Y_FRAG_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree."]
    #[inline(always)]
    pub fn srm_rotate_angle(&mut self) -> SRM_ROTATE_ANGLE_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_ROTATE_ANGLE_W::new(self, 24)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset scaling and rotating engine."]
    #[inline(always)]
    pub fn scal_rotate_rst(&mut self) -> SCAL_ROTATE_RST_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SCAL_ROTATE_RST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Write 1 to enable scaling and rotating engine after parameter is configured."]
    #[inline(always)]
    pub fn scal_rotate_start(&mut self) -> SCAL_ROTATE_START_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SCAL_ROTATE_START_W::new(self, 27)
    }
    #[doc = "Bit 28 - Image mirror in X direction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn srm_mirror_x(&mut self) -> SRM_MIRROR_X_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_MIRROR_X_W::new(self, 28)
    }
    #[doc = "Bit 29 - Image mirror in Y direction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn srm_mirror_y(&mut self) -> SRM_MIRROR_Y_W<'_, SRM_SCAL_ROTATE_SPEC> {
        SRM_MIRROR_Y_W::new(self, 29)
    }
}
#[doc = "Scaling and rotating coefficient register\n\nYou can [`read`](crate::Reg::read) this register and get [`srm_scal_rotate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srm_scal_rotate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRM_SCAL_ROTATE_SPEC;
impl crate::RegisterSpec for SRM_SCAL_ROTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srm_scal_rotate::R`](R) reader structure"]
impl crate::Readable for SRM_SCAL_ROTATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srm_scal_rotate::W`](W) writer structure"]
impl crate::Writable for SRM_SCAL_ROTATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRM_SCAL_ROTATE to value 0x1001"]
impl crate::Resettable for SRM_SCAL_ROTATE_SPEC {
    const RESET_VALUE: u32 = 0x1001;
}
