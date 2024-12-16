#[doc = "Register `AWB_VSCALE` reader"]
pub type R = crate::R<AWB_VSCALE_SPEC>;
#[doc = "Register `AWB_VSCALE` writer"]
pub type W = crate::W<AWB_VSCALE_SPEC>;
#[doc = "Field `AWB_BPOINT` reader - this field configures awb window bottom coordinate"]
pub type AWB_BPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_BPOINT` writer - this field configures awb window bottom coordinate"]
pub type AWB_BPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_TPOINT` reader - this field configures awb window top coordinate"]
pub type AWB_TPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_TPOINT` writer - this field configures awb window top coordinate"]
pub type AWB_TPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures awb window bottom coordinate"]
    #[inline(always)]
    pub fn awb_bpoint(&self) -> AWB_BPOINT_R {
        AWB_BPOINT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures awb window top coordinate"]
    #[inline(always)]
    pub fn awb_tpoint(&self) -> AWB_TPOINT_R {
        AWB_TPOINT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_VSCALE")
            .field("awb_bpoint", &self.awb_bpoint())
            .field("awb_tpoint", &self.awb_tpoint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures awb window bottom coordinate"]
    #[inline(always)]
    pub fn awb_bpoint(&mut self) -> AWB_BPOINT_W<AWB_VSCALE_SPEC> {
        AWB_BPOINT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures awb window top coordinate"]
    #[inline(always)]
    pub fn awb_tpoint(&mut self) -> AWB_TPOINT_W<AWB_VSCALE_SPEC> {
        AWB_TPOINT_W::new(self, 16)
    }
}
#[doc = "v-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_vscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_vscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_VSCALE_SPEC;
impl crate::RegisterSpec for AWB_VSCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_vscale::R`](R) reader structure"]
impl crate::Readable for AWB_VSCALE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_vscale::W`](W) writer structure"]
impl crate::Writable for AWB_VSCALE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_VSCALE to value 0x0437"]
impl crate::Resettable for AWB_VSCALE_SPEC {
    const RESET_VALUE: u32 = 0x0437;
}
