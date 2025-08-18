#[doc = "Register `SDF_3D` reader"]
pub type R = crate::R<SDF_3D_SPEC>;
#[doc = "Register `SDF_3D` writer"]
pub type W = crate::W<SDF_3D_SPEC>;
#[doc = "Field `MODE_3D` reader - NA"]
pub type MODE_3D_R = crate::FieldReader;
#[doc = "Field `MODE_3D` writer - NA"]
pub type MODE_3D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORMAT_3D` reader - NA"]
pub type FORMAT_3D_R = crate::FieldReader;
#[doc = "Field `FORMAT_3D` writer - NA"]
pub type FORMAT_3D_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECOND_VSYNC` reader - NA"]
pub type SECOND_VSYNC_R = crate::BitReader;
#[doc = "Field `SECOND_VSYNC` writer - NA"]
pub type SECOND_VSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIGHT_FIRST` reader - NA"]
pub type RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `RIGHT_FIRST` writer - NA"]
pub type RIGHT_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_3D_CFG` reader - NA"]
pub type SEND_3D_CFG_R = crate::BitReader;
#[doc = "Field `SEND_3D_CFG` writer - NA"]
pub type SEND_3D_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn mode_3d(&self) -> MODE_3D_R {
        MODE_3D_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn format_3d(&self) -> FORMAT_3D_R {
        FORMAT_3D_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn second_vsync(&self) -> SECOND_VSYNC_R {
        SECOND_VSYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn right_first(&self) -> RIGHT_FIRST_R {
        RIGHT_FIRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn send_3d_cfg(&self) -> SEND_3D_CFG_R {
        SEND_3D_CFG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDF_3D")
            .field("mode_3d", &self.mode_3d())
            .field("format_3d", &self.format_3d())
            .field("second_vsync", &self.second_vsync())
            .field("right_first", &self.right_first())
            .field("send_3d_cfg", &self.send_3d_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn mode_3d(&mut self) -> MODE_3D_W<'_, SDF_3D_SPEC> {
        MODE_3D_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn format_3d(&mut self) -> FORMAT_3D_W<'_, SDF_3D_SPEC> {
        FORMAT_3D_W::new(self, 2)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn second_vsync(&mut self) -> SECOND_VSYNC_W<'_, SDF_3D_SPEC> {
        SECOND_VSYNC_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn right_first(&mut self) -> RIGHT_FIRST_W<'_, SDF_3D_SPEC> {
        RIGHT_FIRST_W::new(self, 5)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn send_3d_cfg(&mut self) -> SEND_3D_CFG_W<'_, SDF_3D_SPEC> {
        SEND_3D_CFG_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sdf_3d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdf_3d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDF_3D_SPEC;
impl crate::RegisterSpec for SDF_3D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdf_3d::R`](R) reader structure"]
impl crate::Readable for SDF_3D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdf_3d::W`](W) writer structure"]
impl crate::Writable for SDF_3D_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDF_3D to value 0"]
impl crate::Resettable for SDF_3D_SPEC {}
