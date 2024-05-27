#[doc = "Register `SDF_3D_ACT` reader"]
pub type R = crate::R<SDF_3D_ACT_SPEC>;
#[doc = "Field `MODE_3D_ACT` reader - NA"]
pub type MODE_3D_ACT_R = crate::FieldReader;
#[doc = "Field `FORMAT_3D_ACT` reader - NA"]
pub type FORMAT_3D_ACT_R = crate::FieldReader;
#[doc = "Field `SECOND_VSYNC_ACT` reader - NA"]
pub type SECOND_VSYNC_ACT_R = crate::BitReader;
#[doc = "Field `RIGHT_FIRST_ACT` reader - NA"]
pub type RIGHT_FIRST_ACT_R = crate::BitReader;
#[doc = "Field `SEND_3D_CFG_ACT` reader - NA"]
pub type SEND_3D_CFG_ACT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn mode_3d_act(&self) -> MODE_3D_ACT_R {
        MODE_3D_ACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn format_3d_act(&self) -> FORMAT_3D_ACT_R {
        FORMAT_3D_ACT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn second_vsync_act(&self) -> SECOND_VSYNC_ACT_R {
        SECOND_VSYNC_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn right_first_act(&self) -> RIGHT_FIRST_ACT_R {
        RIGHT_FIRST_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn send_3d_cfg_act(&self) -> SEND_3D_CFG_ACT_R {
        SEND_3D_CFG_ACT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDF_3D_ACT")
            .field("mode_3d_act", &self.mode_3d_act())
            .field("format_3d_act", &self.format_3d_act())
            .field("second_vsync_act", &self.second_vsync_act())
            .field("right_first_act", &self.right_first_act())
            .field("send_3d_cfg_act", &self.send_3d_cfg_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdf_3d_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDF_3D_ACT_SPEC;
impl crate::RegisterSpec for SDF_3D_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdf_3d_act::R`](R) reader structure"]
impl crate::Readable for SDF_3D_ACT_SPEC {}
#[doc = "`reset()` method sets SDF_3D_ACT to value 0"]
impl crate::Resettable for SDF_3D_ACT_SPEC {
    const RESET_VALUE: u32 = 0;
}
