#[doc = "Register `BF_MATRIX_CTRL` reader"]
pub type R = crate::R<BF_MATRIX_CTRL_SPEC>;
#[doc = "Register `BF_MATRIX_CTRL` writer"]
pub type W = crate::W<BF_MATRIX_CTRL_SPEC>;
#[doc = "Field `BF_TAIL_PIXEN_PULSE_TL` reader - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
pub type BF_TAIL_PIXEN_PULSE_TL_R = crate::FieldReader;
#[doc = "Field `BF_TAIL_PIXEN_PULSE_TL` writer - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
pub type BF_TAIL_PIXEN_PULSE_TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BF_TAIL_PIXEN_PULSE_TH` reader - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
pub type BF_TAIL_PIXEN_PULSE_TH_R = crate::FieldReader;
#[doc = "Field `BF_TAIL_PIXEN_PULSE_TH` writer - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
pub type BF_TAIL_PIXEN_PULSE_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BF_PADDING_DATA` reader - this field configures bf matrix padding data"]
pub type BF_PADDING_DATA_R = crate::FieldReader;
#[doc = "Field `BF_PADDING_DATA` writer - this field configures bf matrix padding data"]
pub type BF_PADDING_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BF_PADDING_MODE` reader - this bit configures the padding mode of bf matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type BF_PADDING_MODE_R = crate::BitReader;
#[doc = "Field `BF_PADDING_MODE` writer - this bit configures the padding mode of bf matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type BF_PADDING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn bf_tail_pixen_pulse_tl(&self) -> BF_TAIL_PIXEN_PULSE_TL_R {
        BF_TAIL_PIXEN_PULSE_TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn bf_tail_pixen_pulse_th(&self) -> BF_TAIL_PIXEN_PULSE_TH_R {
        BF_TAIL_PIXEN_PULSE_TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures bf matrix padding data"]
    #[inline(always)]
    pub fn bf_padding_data(&self) -> BF_PADDING_DATA_R {
        BF_PADDING_DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of bf matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    pub fn bf_padding_mode(&self) -> BF_PADDING_MODE_R {
        BF_PADDING_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BF_MATRIX_CTRL")
            .field(
                "bf_tail_pixen_pulse_tl",
                &format_args!("{}", self.bf_tail_pixen_pulse_tl().bits()),
            )
            .field(
                "bf_tail_pixen_pulse_th",
                &format_args!("{}", self.bf_tail_pixen_pulse_th().bits()),
            )
            .field(
                "bf_padding_data",
                &format_args!("{}", self.bf_padding_data().bits()),
            )
            .field(
                "bf_padding_mode",
                &format_args!("{}", self.bf_padding_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BF_MATRIX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    #[must_use]
    pub fn bf_tail_pixen_pulse_tl(&mut self) -> BF_TAIL_PIXEN_PULSE_TL_W<BF_MATRIX_CTRL_SPEC> {
        BF_TAIL_PIXEN_PULSE_TL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_bf_tail_pixen_pulse_th!=0 and reg_bf_tail_pixen_pulse_tl!=0 and reg_bf_tail_pixen_pulse_th &lt; reg_bf_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    #[must_use]
    pub fn bf_tail_pixen_pulse_th(&mut self) -> BF_TAIL_PIXEN_PULSE_TH_W<BF_MATRIX_CTRL_SPEC> {
        BF_TAIL_PIXEN_PULSE_TH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures bf matrix padding data"]
    #[inline(always)]
    #[must_use]
    pub fn bf_padding_data(&mut self) -> BF_PADDING_DATA_W<BF_MATRIX_CTRL_SPEC> {
        BF_PADDING_DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of bf matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    #[must_use]
    pub fn bf_padding_mode(&mut self) -> BF_PADDING_MODE_W<BF_MATRIX_CTRL_SPEC> {
        BF_PADDING_MODE_W::new(self, 24)
    }
}
#[doc = "bf pix2matrix ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bf_matrix_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_matrix_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BF_MATRIX_CTRL_SPEC;
impl crate::RegisterSpec for BF_MATRIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bf_matrix_ctrl::R`](R) reader structure"]
impl crate::Readable for BF_MATRIX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bf_matrix_ctrl::W`](W) writer structure"]
impl crate::Writable for BF_MATRIX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BF_MATRIX_CTRL to value 0"]
impl crate::Resettable for BF_MATRIX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
