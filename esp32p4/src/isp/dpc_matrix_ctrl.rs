#[doc = "Register `DPC_MATRIX_CTRL` reader"]
pub type R = crate::R<DPC_MATRIX_CTRL_SPEC>;
#[doc = "Register `DPC_MATRIX_CTRL` writer"]
pub type W = crate::W<DPC_MATRIX_CTRL_SPEC>;
#[doc = "Field `DPC_TAIL_PIXEN_PULSE_TL` reader - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DPC_TAIL_PIXEN_PULSE_TL_R = crate::FieldReader;
#[doc = "Field `DPC_TAIL_PIXEN_PULSE_TL` writer - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DPC_TAIL_PIXEN_PULSE_TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_TAIL_PIXEN_PULSE_TH` reader - matrix tail pixen high level threshold, must < hnum-1, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DPC_TAIL_PIXEN_PULSE_TH_R = crate::FieldReader;
#[doc = "Field `DPC_TAIL_PIXEN_PULSE_TH` writer - matrix tail pixen high level threshold, must < hnum-1, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
pub type DPC_TAIL_PIXEN_PULSE_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_PADDING_DATA` reader - this field configures dpc matrix padding data"]
pub type DPC_PADDING_DATA_R = crate::FieldReader;
#[doc = "Field `DPC_PADDING_DATA` writer - this field configures dpc matrix padding data"]
pub type DPC_PADDING_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_PADDING_MODE` reader - this bit configures the padding mode of dpc matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type DPC_PADDING_MODE_R = crate::BitReader;
#[doc = "Field `DPC_PADDING_MODE` writer - this bit configures the padding mode of dpc matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type DPC_PADDING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn dpc_tail_pixen_pulse_tl(&self) -> DPC_TAIL_PIXEN_PULSE_TL_R {
        DPC_TAIL_PIXEN_PULSE_TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must < hnum-1, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn dpc_tail_pixen_pulse_th(&self) -> DPC_TAIL_PIXEN_PULSE_TH_R {
        DPC_TAIL_PIXEN_PULSE_TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures dpc matrix padding data"]
    #[inline(always)]
    pub fn dpc_padding_data(&self) -> DPC_PADDING_DATA_R {
        DPC_PADDING_DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of dpc matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    pub fn dpc_padding_mode(&self) -> DPC_PADDING_MODE_R {
        DPC_PADDING_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPC_MATRIX_CTRL")
            .field("dpc_tail_pixen_pulse_tl", &self.dpc_tail_pixen_pulse_tl())
            .field("dpc_tail_pixen_pulse_th", &self.dpc_tail_pixen_pulse_th())
            .field("dpc_padding_data", &self.dpc_padding_data())
            .field("dpc_padding_mode", &self.dpc_padding_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - matrix tail pixen low level threshold, should not to large to prevent expanding to next frame, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn dpc_tail_pixen_pulse_tl(&mut self) -> DPC_TAIL_PIXEN_PULSE_TL_W<DPC_MATRIX_CTRL_SPEC> {
        DPC_TAIL_PIXEN_PULSE_TL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - matrix tail pixen high level threshold, must < hnum-1, only reg_dpc_tail_pixen_pulse_th!=0 and reg_dpc_tail_pixen_pulse_tl!=0 and reg_dpc_tail_pixen_pulse_th < reg_dpc_tail_pixen_pulse_tl will enable tail pulse function"]
    #[inline(always)]
    pub fn dpc_tail_pixen_pulse_th(&mut self) -> DPC_TAIL_PIXEN_PULSE_TH_W<DPC_MATRIX_CTRL_SPEC> {
        DPC_TAIL_PIXEN_PULSE_TH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures dpc matrix padding data"]
    #[inline(always)]
    pub fn dpc_padding_data(&mut self) -> DPC_PADDING_DATA_W<DPC_MATRIX_CTRL_SPEC> {
        DPC_PADDING_DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - this bit configures the padding mode of dpc matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    pub fn dpc_padding_mode(&mut self) -> DPC_PADDING_MODE_W<DPC_MATRIX_CTRL_SPEC> {
        DPC_PADDING_MODE_W::new(self, 24)
    }
}
#[doc = "dpc pix2matrix ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_matrix_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_matrix_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPC_MATRIX_CTRL_SPEC;
impl crate::RegisterSpec for DPC_MATRIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_matrix_ctrl::R`](R) reader structure"]
impl crate::Readable for DPC_MATRIX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpc_matrix_ctrl::W`](W) writer structure"]
impl crate::Writable for DPC_MATRIX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPC_MATRIX_CTRL to value 0"]
impl crate::Resettable for DPC_MATRIX_CTRL_SPEC {}
