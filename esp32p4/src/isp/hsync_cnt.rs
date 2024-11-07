#[doc = "Register `HSYNC_CNT` reader"]
pub type R = crate::R<HSYNC_CNT_SPEC>;
#[doc = "Register `HSYNC_CNT` writer"]
pub type W = crate::W<HSYNC_CNT_SPEC>;
#[doc = "Field `HSYNC_CNT` reader - this field configures the number of clock before hsync and after vsync and line_end when decodes pix data from idi to isp"]
pub type HSYNC_CNT_R = crate::FieldReader;
#[doc = "Field `HSYNC_CNT` writer - this field configures the number of clock before hsync and after vsync and line_end when decodes pix data from idi to isp"]
pub type HSYNC_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the number of clock before hsync and after vsync and line_end when decodes pix data from idi to isp"]
    #[inline(always)]
    pub fn hsync_cnt(&self) -> HSYNC_CNT_R {
        HSYNC_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSYNC_CNT")
            .field("hsync_cnt", &self.hsync_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the number of clock before hsync and after vsync and line_end when decodes pix data from idi to isp"]
    #[inline(always)]
    pub fn hsync_cnt(&mut self) -> HSYNC_CNT_W<HSYNC_CNT_SPEC> {
        HSYNC_CNT_W::new(self, 0)
    }
}
#[doc = "header hsync interval control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsync_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsync_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSYNC_CNT_SPEC;
impl crate::RegisterSpec for HSYNC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsync_cnt::R`](R) reader structure"]
impl crate::Readable for HSYNC_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsync_cnt::W`](W) writer structure"]
impl crate::Writable for HSYNC_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSYNC_CNT to value 0x07"]
impl crate::Resettable for HSYNC_CNT_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
