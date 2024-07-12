#[doc = "Register `GDMA_CTRL` reader"]
pub type R = crate::R<GDMA_CTRL_SPEC>;
#[doc = "Register `GDMA_CTRL` writer"]
pub type W = crate::W<GDMA_CTRL_SPEC>;
#[doc = "Field `DEBUG_CH_NUM` reader - N/A"]
pub type DEBUG_CH_NUM_R = crate::FieldReader;
#[doc = "Field `DEBUG_CH_NUM` writer - N/A"]
pub type DEBUG_CH_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn debug_ch_num(&self) -> DEBUG_CH_NUM_R {
        DEBUG_CH_NUM_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CTRL")
            .field("debug_ch_num", &self.debug_ch_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ch_num(&mut self) -> DEBUG_CH_NUM_W<GDMA_CTRL_SPEC> {
        DEBUG_CH_NUM_W::new(self, 0)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDMA_CTRL_SPEC;
impl crate::RegisterSpec for GDMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_ctrl::R`](R) reader structure"]
impl crate::Readable for GDMA_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdma_ctrl::W`](W) writer structure"]
impl crate::Writable for GDMA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_CTRL to value 0"]
impl crate::Resettable for GDMA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
