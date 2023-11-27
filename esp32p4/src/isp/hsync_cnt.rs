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
            .field("hsync_cnt", &format_args!("{}", self.hsync_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSYNC_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the number of clock before hsync and after vsync and line_end when decodes pix data from idi to isp"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_cnt(&mut self) -> HSYNC_CNT_W<HSYNC_CNT_SPEC> {
        HSYNC_CNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "header hsync interval control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsync_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsync_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSYNC_CNT_SPEC;
impl crate::RegisterSpec for HSYNC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsync_cnt::R`](R) reader structure"]
impl crate::Readable for HSYNC_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsync_cnt::W`](W) writer structure"]
impl crate::Writable for HSYNC_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSYNC_CNT to value 0x07"]
impl crate::Resettable for HSYNC_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
