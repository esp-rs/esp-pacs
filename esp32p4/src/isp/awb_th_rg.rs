#[doc = "Register `AWB_TH_RG` reader"]
pub type R = crate::R<AWB_TH_RG_SPEC>;
#[doc = "Register `AWB_TH_RG` writer"]
pub type W = crate::W<AWB_TH_RG_SPEC>;
#[doc = "Field `AWB_MIN_RG` reader - this field configures lower threshold of r/g, 2bit integer and 8bit fraction"]
pub type AWB_MIN_RG_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_MIN_RG` writer - this field configures lower threshold of r/g, 2bit integer and 8bit fraction"]
pub type AWB_MIN_RG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AWB_MAX_RG` reader - this field configures upper threshold of r/g, 2bit integer and 8bit fraction"]
pub type AWB_MAX_RG_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_MAX_RG` writer - this field configures upper threshold of r/g, 2bit integer and 8bit fraction"]
pub type AWB_MAX_RG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - this field configures lower threshold of r/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_min_rg(&self) -> AWB_MIN_RG_R {
        AWB_MIN_RG_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of r/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_max_rg(&self) -> AWB_MAX_RG_R {
        AWB_MAX_RG_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_TH_RG")
            .field("awb_min_rg", &self.awb_min_rg())
            .field("awb_max_rg", &self.awb_max_rg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - this field configures lower threshold of r/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_min_rg(&mut self) -> AWB_MIN_RG_W<AWB_TH_RG_SPEC> {
        AWB_MIN_RG_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of r/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_max_rg(&mut self) -> AWB_MAX_RG_W<AWB_TH_RG_SPEC> {
        AWB_MAX_RG_W::new(self, 16)
    }
}
#[doc = "awb r/g threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_th_rg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_th_rg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_TH_RG_SPEC;
impl crate::RegisterSpec for AWB_TH_RG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_th_rg::R`](R) reader structure"]
impl crate::Readable for AWB_TH_RG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_th_rg::W`](W) writer structure"]
impl crate::Writable for AWB_TH_RG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_TH_RG to value 0x03ff_0000"]
impl crate::Resettable for AWB_TH_RG_SPEC {
    const RESET_VALUE: u32 = 0x03ff_0000;
}
