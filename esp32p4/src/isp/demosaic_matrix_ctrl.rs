#[doc = "Register `DEMOSAIC_MATRIX_CTRL` reader"]
pub type R = crate::R<DEMOSAIC_MATRIX_CTRL_SPEC>;
#[doc = "Register `DEMOSAIC_MATRIX_CTRL` writer"]
pub type W = crate::W<DEMOSAIC_MATRIX_CTRL_SPEC>;
#[doc = "Field `DEMOSAIC_TAIL_PIXEN_PULSE_TL` reader - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DEMOSAIC_TAIL_PIXEN_PULSE_TL_R = crate::FieldReader;
#[doc = "Field `DEMOSAIC_TAIL_PIXEN_PULSE_TL` writer - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DEMOSAIC_TAIL_PIXEN_PULSE_TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEMOSAIC_TAIL_PIXEN_PULSE_TH` reader - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DEMOSAIC_TAIL_PIXEN_PULSE_TH_R = crate::FieldReader;
#[doc = "Field `DEMOSAIC_TAIL_PIXEN_PULSE_TH` writer - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DEMOSAIC_TAIL_PIXEN_PULSE_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEMOSAIC_PADDING_DATA` reader - this field configures demosaic matrix padding data"]
pub type DEMOSAIC_PADDING_DATA_R = crate::FieldReader;
#[doc = "Field `DEMOSAIC_PADDING_DATA` writer - this field configures demosaic matrix padding data"]
pub type DEMOSAIC_PADDING_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEMOSAIC_PADDING_MODE` reader - this bit configures the padding mode of demosaic matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type DEMOSAIC_PADDING_MODE_R = crate::BitReader;
#[doc = "Field `DEMOSAIC_PADDING_MODE` writer - this bit configures the padding mode of demosaic matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type DEMOSAIC_PADDING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn demosaic_tail_pixen_pulse_tl(&self) -> DEMOSAIC_TAIL_PIXEN_PULSE_TL_R {
        DEMOSAIC_TAIL_PIXEN_PULSE_TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn demosaic_tail_pixen_pulse_th(&self) -> DEMOSAIC_TAIL_PIXEN_PULSE_TH_R {
        DEMOSAIC_TAIL_PIXEN_PULSE_TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures demosaic matrix padding data"]
    #[inline(always)]
    pub fn demosaic_padding_data(&self) -> DEMOSAIC_PADDING_DATA_R {
        DEMOSAIC_PADDING_DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of demosaic matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    pub fn demosaic_padding_mode(&self) -> DEMOSAIC_PADDING_MODE_R {
        DEMOSAIC_PADDING_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMOSAIC_MATRIX_CTRL")
            .field(
                "demosaic_tail_pixen_pulse_tl",
                &format_args!("{}", self.demosaic_tail_pixen_pulse_tl().bits()),
            )
            .field(
                "demosaic_tail_pixen_pulse_th",
                &format_args!("{}", self.demosaic_tail_pixen_pulse_th().bits()),
            )
            .field(
                "demosaic_padding_data",
                &format_args!("{}", self.demosaic_padding_data().bits()),
            )
            .field(
                "demosaic_padding_mode",
                &format_args!("{}", self.demosaic_padding_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEMOSAIC_MATRIX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_tail_pixen_pulse_tl(
        &mut self,
    ) -> DEMOSAIC_TAIL_PIXEN_PULSE_TL_W<DEMOSAIC_MATRIX_CTRL_SPEC> {
        DEMOSAIC_TAIL_PIXEN_PULSE_TL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must &lt; hnum-1, only reg_demosaic_tail_pixen_pulse_th!=0 and reg_demosaic_tail_pixen_pulse_tl!=0 and reg_demosaic_tail_pixen_pulse_th &lt; reg_demosaic_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_tail_pixen_pulse_th(
        &mut self,
    ) -> DEMOSAIC_TAIL_PIXEN_PULSE_TH_W<DEMOSAIC_MATRIX_CTRL_SPEC> {
        DEMOSAIC_TAIL_PIXEN_PULSE_TH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures demosaic matrix padding data"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_padding_data(&mut self) -> DEMOSAIC_PADDING_DATA_W<DEMOSAIC_MATRIX_CTRL_SPEC> {
        DEMOSAIC_PADDING_DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of demosaic matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_padding_mode(&mut self) -> DEMOSAIC_PADDING_MODE_W<DEMOSAIC_MATRIX_CTRL_SPEC> {
        DEMOSAIC_PADDING_MODE_W::new(self, 24)
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
#[doc = "demosaic pix2matrix ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demosaic_matrix_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic_matrix_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEMOSAIC_MATRIX_CTRL_SPEC;
impl crate::RegisterSpec for DEMOSAIC_MATRIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demosaic_matrix_ctrl::R`](R) reader structure"]
impl crate::Readable for DEMOSAIC_MATRIX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`demosaic_matrix_ctrl::W`](W) writer structure"]
impl crate::Writable for DEMOSAIC_MATRIX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMOSAIC_MATRIX_CTRL to value 0"]
impl crate::Resettable for DEMOSAIC_MATRIX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
