#[doc = "Register `PLC_CONF0` reader"]
pub type R = crate::R<PLC_CONF0_SPEC>;
#[doc = "Register `PLC_CONF0` writer"]
pub type W = crate::W<PLC_CONF0_SPEC>;
#[doc = "Field `GOOD_PACK_MAX` reader - "]
pub type GOOD_PACK_MAX_R = crate::FieldReader;
#[doc = "Field `GOOD_PACK_MAX` writer - "]
pub type GOOD_PACK_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `N_ERR_SEG` reader - "]
pub type N_ERR_SEG_R = crate::FieldReader;
#[doc = "Field `N_ERR_SEG` writer - "]
pub type N_ERR_SEG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SHIFT_RATE` reader - "]
pub type SHIFT_RATE_R = crate::FieldReader;
#[doc = "Field `SHIFT_RATE` writer - "]
pub type SHIFT_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX_SLIDE_SAMPLE` reader - "]
pub type MAX_SLIDE_SAMPLE_R = crate::FieldReader;
#[doc = "Field `MAX_SLIDE_SAMPLE` writer - "]
pub type MAX_SLIDE_SAMPLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PACK_LEN_8K` reader - "]
pub type PACK_LEN_8K_R = crate::FieldReader;
#[doc = "Field `PACK_LEN_8K` writer - "]
pub type PACK_LEN_8K_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `N_MIN_ERR` reader - "]
pub type N_MIN_ERR_R = crate::FieldReader;
#[doc = "Field `N_MIN_ERR` writer - "]
pub type N_MIN_ERR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&self) -> GOOD_PACK_MAX_R {
        GOOD_PACK_MAX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&self) -> N_ERR_SEG_R {
        N_ERR_SEG_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&self) -> SHIFT_RATE_R {
        SHIFT_RATE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&self) -> MAX_SLIDE_SAMPLE_R {
        MAX_SLIDE_SAMPLE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&self) -> PACK_LEN_8K_R {
        PACK_LEN_8K_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&self) -> N_MIN_ERR_R {
        N_MIN_ERR_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLC_CONF0")
            .field("good_pack_max", &self.good_pack_max())
            .field("n_err_seg", &self.n_err_seg())
            .field("shift_rate", &self.shift_rate())
            .field("max_slide_sample", &self.max_slide_sample())
            .field("pack_len_8k", &self.pack_len_8k())
            .field("n_min_err", &self.n_min_err())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn good_pack_max(&mut self) -> GOOD_PACK_MAX_W<PLC_CONF0_SPEC> {
        GOOD_PACK_MAX_W::new(self, 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn n_err_seg(&mut self) -> N_ERR_SEG_W<PLC_CONF0_SPEC> {
        N_ERR_SEG_W::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn shift_rate(&mut self) -> SHIFT_RATE_W<PLC_CONF0_SPEC> {
        SHIFT_RATE_W::new(self, 9)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    #[must_use]
    pub fn max_slide_sample(&mut self) -> MAX_SLIDE_SAMPLE_W<PLC_CONF0_SPEC> {
        MAX_SLIDE_SAMPLE_W::new(self, 12)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn pack_len_8k(&mut self) -> PACK_LEN_8K_W<PLC_CONF0_SPEC> {
        PACK_LEN_8K_W::new(self, 20)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn n_min_err(&mut self) -> N_MIN_ERR_W<PLC_CONF0_SPEC> {
        N_MIN_ERR_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLC_CONF0_SPEC;
impl crate::RegisterSpec for PLC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plc_conf0::R`](R) reader structure"]
impl crate::Readable for PLC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plc_conf0::W`](W) writer structure"]
impl crate::Writable for PLC_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLC_CONF0 to value 0x08a8_0339"]
impl crate::Resettable for PLC_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x08a8_0339;
}
