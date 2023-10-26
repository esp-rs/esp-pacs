#[doc = "Register `PLC_CONF0` reader"]
pub type R = crate::R<PLC_CONF0_SPEC>;
#[doc = "Register `PLC_CONF0` writer"]
pub type W = crate::W<PLC_CONF0_SPEC>;
#[doc = "Field `GOOD_PACK_MAX` reader - "]
pub type GOOD_PACK_MAX_R = crate::FieldReader;
#[doc = "Field `GOOD_PACK_MAX` writer - "]
pub type GOOD_PACK_MAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `N_ERR_SEG` reader - "]
pub type N_ERR_SEG_R = crate::FieldReader;
#[doc = "Field `N_ERR_SEG` writer - "]
pub type N_ERR_SEG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SHIFT_RATE` reader - "]
pub type SHIFT_RATE_R = crate::FieldReader;
#[doc = "Field `SHIFT_RATE` writer - "]
pub type SHIFT_RATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MAX_SLIDE_SAMPLE` reader - "]
pub type MAX_SLIDE_SAMPLE_R = crate::FieldReader;
#[doc = "Field `MAX_SLIDE_SAMPLE` writer - "]
pub type MAX_SLIDE_SAMPLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PACK_LEN_8K` reader - "]
pub type PACK_LEN_8K_R = crate::FieldReader;
#[doc = "Field `PACK_LEN_8K` writer - "]
pub type PACK_LEN_8K_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `N_MIN_ERR` reader - "]
pub type N_MIN_ERR_R = crate::FieldReader;
#[doc = "Field `N_MIN_ERR` writer - "]
pub type N_MIN_ERR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
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
            .field(
                "good_pack_max",
                &format_args!("{}", self.good_pack_max().bits()),
            )
            .field("n_err_seg", &format_args!("{}", self.n_err_seg().bits()))
            .field("shift_rate", &format_args!("{}", self.shift_rate().bits()))
            .field(
                "max_slide_sample",
                &format_args!("{}", self.max_slide_sample().bits()),
            )
            .field(
                "pack_len_8k",
                &format_args!("{}", self.pack_len_8k().bits()),
            )
            .field("n_min_err", &format_args!("{}", self.n_min_err().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn good_pack_max(&mut self) -> GOOD_PACK_MAX_W<PLC_CONF0_SPEC, 0> {
        GOOD_PACK_MAX_W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn n_err_seg(&mut self) -> N_ERR_SEG_W<PLC_CONF0_SPEC, 6> {
        N_ERR_SEG_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn shift_rate(&mut self) -> SHIFT_RATE_W<PLC_CONF0_SPEC, 9> {
        SHIFT_RATE_W::new(self)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    #[must_use]
    pub fn max_slide_sample(&mut self) -> MAX_SLIDE_SAMPLE_W<PLC_CONF0_SPEC, 12> {
        MAX_SLIDE_SAMPLE_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn pack_len_8k(&mut self) -> PACK_LEN_8K_W<PLC_CONF0_SPEC, 20> {
        PACK_LEN_8K_W::new(self)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn n_min_err(&mut self) -> N_MIN_ERR_W<PLC_CONF0_SPEC, 25> {
        N_MIN_ERR_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLC_CONF0_SPEC;
impl crate::RegisterSpec for PLC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plc_conf0::R`](R) reader structure"]
impl crate::Readable for PLC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plc_conf0::W`](W) writer structure"]
impl crate::Writable for PLC_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLC_CONF0 to value 0x08a8_0339"]
impl crate::Resettable for PLC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08a8_0339;
}
