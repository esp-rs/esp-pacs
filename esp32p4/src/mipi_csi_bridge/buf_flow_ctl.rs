#[doc = "Register `BUF_FLOW_CTL` reader"]
pub type R = crate::R<BUF_FLOW_CTL_SPEC>;
#[doc = "Register `BUF_FLOW_CTL` writer"]
pub type W = crate::W<BUF_FLOW_CTL_SPEC>;
#[doc = "Field `CSI_BUF_AFULL_THRD` reader - buffer almost full threshold."]
pub type CSI_BUF_AFULL_THRD_R = crate::FieldReader<u16>;
#[doc = "Field `CSI_BUF_AFULL_THRD` writer - buffer almost full threshold."]
pub type CSI_BUF_AFULL_THRD_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CSI_BUF_DEPTH` reader - buffer data count."]
pub type CSI_BUF_DEPTH_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - buffer almost full threshold."]
    #[inline(always)]
    pub fn csi_buf_afull_thrd(&self) -> CSI_BUF_AFULL_THRD_R {
        CSI_BUF_AFULL_THRD_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - buffer data count."]
    #[inline(always)]
    pub fn csi_buf_depth(&self) -> CSI_BUF_DEPTH_R {
        CSI_BUF_DEPTH_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF_FLOW_CTL")
            .field("csi_buf_afull_thrd", &self.csi_buf_afull_thrd())
            .field("csi_buf_depth", &self.csi_buf_depth())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - buffer almost full threshold."]
    #[inline(always)]
    #[must_use]
    pub fn csi_buf_afull_thrd(&mut self) -> CSI_BUF_AFULL_THRD_W<BUF_FLOW_CTL_SPEC> {
        CSI_BUF_AFULL_THRD_W::new(self, 0)
    }
}
#[doc = "csi bridge buffer control.\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_flow_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_flow_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_FLOW_CTL_SPEC;
impl crate::RegisterSpec for BUF_FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_flow_ctl::R`](R) reader structure"]
impl crate::Readable for BUF_FLOW_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_flow_ctl::W`](W) writer structure"]
impl crate::Writable for BUF_FLOW_CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF_FLOW_CTL to value 0x07f8"]
impl crate::Resettable for BUF_FLOW_CTL_SPEC {
    const RESET_VALUE: u32 = 0x07f8;
}
