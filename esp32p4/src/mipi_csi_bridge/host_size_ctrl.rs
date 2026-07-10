#[doc = "Register `HOST_SIZE_CTRL` reader"]
pub type R = crate::R<HOST_SIZE_CTRL_SPEC>;
#[doc = "Register `HOST_SIZE_CTRL` writer"]
pub type W = crate::W<HOST_SIZE_CTRL_SPEC>;
#[doc = "Field `CSI_HOST_CM_VNUM` reader - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CSI_HOST_CM_VNUM_R = crate::FieldReader<u16>;
#[doc = "Field `CSI_HOST_CM_VNUM` writer - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CSI_HOST_CM_VNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CSI_HOST_CM_HNUM` reader - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CSI_HOST_CM_HNUM_R = crate::FieldReader<u16>;
#[doc = "Field `CSI_HOST_CM_HNUM` writer - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
pub type CSI_HOST_CM_HNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_vnum(&self) -> CSI_HOST_CM_VNUM_R {
        CSI_HOST_CM_VNUM_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_hnum(&self) -> CSI_HOST_CM_HNUM_R {
        CSI_HOST_CM_HNUM_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SIZE_CTRL")
            .field("csi_host_cm_vnum", &self.csi_host_cm_vnum())
            .field("csi_host_cm_hnum", &self.csi_host_cm_hnum())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures idi32 image size in y-direction, row_num - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_vnum(&mut self) -> CSI_HOST_CM_VNUM_W<'_, HOST_SIZE_CTRL_SPEC> {
        CSI_HOST_CM_VNUM_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - Configures idi32 image size in x-direction, line_pix_num*bits_per_pix/32 - 1, valid only when yuv422_to_yuv420_en = 1"]
    #[inline(always)]
    pub fn csi_host_cm_hnum(&mut self) -> CSI_HOST_CM_HNUM_W<'_, HOST_SIZE_CTRL_SPEC> {
        CSI_HOST_CM_HNUM_W::new(self, 12)
    }
}
#[doc = "CSI HOST color mode convert configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_size_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_size_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SIZE_CTRL_SPEC;
impl crate::RegisterSpec for HOST_SIZE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_size_ctrl::R`](R) reader structure"]
impl crate::Readable for HOST_SIZE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_size_ctrl::W`](W) writer structure"]
impl crate::Writable for HOST_SIZE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_SIZE_CTRL to value 0"]
impl crate::Resettable for HOST_SIZE_CTRL_SPEC {}
