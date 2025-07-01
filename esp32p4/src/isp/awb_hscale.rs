#[doc = "Register `AWB_HSCALE` reader"]
pub type R = crate::R<AWB_HSCALE_SPEC>;
#[doc = "Register `AWB_HSCALE` writer"]
pub type W = crate::W<AWB_HSCALE_SPEC>;
#[doc = "Field `AWB_RPOINT` reader - this field configures awb window right coordinate"]
pub type AWB_RPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_RPOINT` writer - this field configures awb window right coordinate"]
pub type AWB_RPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_LPOINT` reader - this field configures awb window left coordinate"]
pub type AWB_LPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_LPOINT` writer - this field configures awb window left coordinate"]
pub type AWB_LPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures awb window right coordinate"]
    #[inline(always)]
    pub fn awb_rpoint(&self) -> AWB_RPOINT_R {
        AWB_RPOINT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures awb window left coordinate"]
    #[inline(always)]
    pub fn awb_lpoint(&self) -> AWB_LPOINT_R {
        AWB_LPOINT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_HSCALE")
            .field("awb_rpoint", &self.awb_rpoint())
            .field("awb_lpoint", &self.awb_lpoint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures awb window right coordinate"]
    #[inline(always)]
    pub fn awb_rpoint(&mut self) -> AWB_RPOINT_W<AWB_HSCALE_SPEC> {
        AWB_RPOINT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures awb window left coordinate"]
    #[inline(always)]
    pub fn awb_lpoint(&mut self) -> AWB_LPOINT_W<AWB_HSCALE_SPEC> {
        AWB_LPOINT_W::new(self, 16)
    }
}
#[doc = "h-scale of awb window\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_hscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_hscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_HSCALE_SPEC;
impl crate::RegisterSpec for AWB_HSCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_hscale::R`](R) reader structure"]
impl crate::Readable for AWB_HSCALE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_hscale::W`](W) writer structure"]
impl crate::Writable for AWB_HSCALE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_HSCALE to value 0x077f"]
impl crate::Resettable for AWB_HSCALE_SPEC {
    const RESET_VALUE: u32 = 0x077f;
}
