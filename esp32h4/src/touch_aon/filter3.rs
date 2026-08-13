#[doc = "Register `FILTER3` reader"]
pub type R = crate::R<FILTER3_SPEC>;
#[doc = "Register `FILTER3` writer"]
pub type W = crate::W<FILTER3_SPEC>;
#[doc = "Field `TOUCH_BENCHMARK_SW` reader - need_des"]
pub type TOUCH_BENCHMARK_SW_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_BENCHMARK_SW` writer - need_des"]
pub type TOUCH_BENCHMARK_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_UPDATE_BENCHMARK_SW` writer - need_des"]
pub type TOUCH_UPDATE_BENCHMARK_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_UPDATE_BENCHMARK_FREQ_SEL` reader - Reserved"]
pub type TOUCH_UPDATE_BENCHMARK_FREQ_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BENCHMARK_FREQ_SEL` writer - Reserved"]
pub type TOUCH_UPDATE_BENCHMARK_FREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_UPDATE_BENCHMARK_PAD_SEL` reader - Reserved"]
pub type TOUCH_UPDATE_BENCHMARK_PAD_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BENCHMARK_PAD_SEL` writer - Reserved"]
pub type TOUCH_UPDATE_BENCHMARK_PAD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_benchmark_sw(&self) -> TOUCH_BENCHMARK_SW_R {
        TOUCH_BENCHMARK_SW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    pub fn touch_update_benchmark_freq_sel(&self) -> TOUCH_UPDATE_BENCHMARK_FREQ_SEL_R {
        TOUCH_UPDATE_BENCHMARK_FREQ_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:22 - Reserved"]
    #[inline(always)]
    pub fn touch_update_benchmark_pad_sel(&self) -> TOUCH_UPDATE_BENCHMARK_PAD_SEL_R {
        TOUCH_UPDATE_BENCHMARK_PAD_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER3")
            .field("touch_benchmark_sw", &self.touch_benchmark_sw())
            .field(
                "touch_update_benchmark_freq_sel",
                &self.touch_update_benchmark_freq_sel(),
            )
            .field(
                "touch_update_benchmark_pad_sel",
                &self.touch_update_benchmark_pad_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_benchmark_sw(&mut self) -> TOUCH_BENCHMARK_SW_W<'_, FILTER3_SPEC> {
        TOUCH_BENCHMARK_SW_W::new(self, 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_update_benchmark_sw(&mut self) -> TOUCH_UPDATE_BENCHMARK_SW_W<'_, FILTER3_SPEC> {
        TOUCH_UPDATE_BENCHMARK_SW_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    pub fn touch_update_benchmark_freq_sel(
        &mut self,
    ) -> TOUCH_UPDATE_BENCHMARK_FREQ_SEL_W<'_, FILTER3_SPEC> {
        TOUCH_UPDATE_BENCHMARK_FREQ_SEL_W::new(self, 17)
    }
    #[doc = "Bits 19:22 - Reserved"]
    #[inline(always)]
    pub fn touch_update_benchmark_pad_sel(
        &mut self,
    ) -> TOUCH_UPDATE_BENCHMARK_PAD_SEL_W<'_, FILTER3_SPEC> {
        TOUCH_UPDATE_BENCHMARK_PAD_SEL_W::new(self, 19)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`filter3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER3_SPEC;
impl crate::RegisterSpec for FILTER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter3::R`](R) reader structure"]
impl crate::Readable for FILTER3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter3::W`](W) writer structure"]
impl crate::Writable for FILTER3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER3 to value 0x007e_0000"]
impl crate::Resettable for FILTER3_SPEC {
    const RESET_VALUE: u32 = 0x007e_0000;
}
