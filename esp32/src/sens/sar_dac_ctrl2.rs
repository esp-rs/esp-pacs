#[doc = "Register `SAR_DAC_CTRL2` reader"]
pub type R = crate::R<SAR_DAC_CTRL2_SPEC>;
#[doc = "Register `SAR_DAC_CTRL2` writer"]
pub type W = crate::W<SAR_DAC_CTRL2_SPEC>;
#[doc = "Field `DAC_DC(1-2)` reader - DC offset for DAC%s CW generator"]
pub type DAC_DC_R = crate::FieldReader;
#[doc = "Field `DAC_DC(1-2)` writer - DC offset for DAC%s CW generator"]
pub type DAC_DC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAC_SCALE(1-2)` reader - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE_R = crate::FieldReader;
#[doc = "Field `DAC_SCALE(1-2)` writer - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub type DAC_SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_INV(1-2)` reader - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV_R = crate::FieldReader;
#[doc = "Field `DAC_INV(1-2)` writer - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub type DAC_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_CW_EN(1-2)` reader - 1: to select CW generator as source to PDAC%s_DAC\\[7:0\\] 0: to select register reg_pdac%s_dac\\[7:0\\] as source to PDAC%s_DAC\\[7:0\\]"]
pub type DAC_CW_EN_R = crate::BitReader;
#[doc = "Field `DAC_CW_EN(1-2)` writer - 1: to select CW generator as source to PDAC%s_DAC\\[7:0\\] 0: to select register reg_pdac%s_dac\\[7:0\\] as source to PDAC%s_DAC\\[7:0\\]"]
pub type DAC_CW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DC offset for DAC(1-2) CW generator"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_DC1` field.</div>"]
    #[inline(always)]
    pub fn dac_dc(&self, n: u8) -> DAC_DC_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_DC_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DC offset for DAC(1-2) CW generator"]
    #[inline(always)]
    pub fn dac_dc_iter(&self) -> impl Iterator<Item = DAC_DC_R> + '_ {
        (0..2).map(move |n| DAC_DC_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&self) -> DAC_DC_R {
        DAC_DC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&self) -> DAC_DC_R {
        DAC_DC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_SCALE1` field.</div>"]
    #[inline(always)]
    pub fn dac_scale(&self, n: u8) -> DAC_SCALE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_SCALE_R::new(((self.bits >> (n * 2 + 16)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale_iter(&self) -> impl Iterator<Item = DAC_SCALE_R> + '_ {
        (0..2).map(move |n| DAC_SCALE_R::new(((self.bits >> (n * 2 + 16)) & 3) as u8))
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&self) -> DAC_SCALE_R {
        DAC_SCALE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&self) -> DAC_SCALE_R {
        DAC_SCALE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_INV1` field.</div>"]
    #[inline(always)]
    pub fn dac_inv(&self, n: u8) -> DAC_INV_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_INV_R::new(((self.bits >> (n * 2 + 20)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv_iter(&self) -> impl Iterator<Item = DAC_INV_R> + '_ {
        (0..2).map(move |n| DAC_INV_R::new(((self.bits >> (n * 2 + 20)) & 3) as u8))
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&self) -> DAC_INV_R {
        DAC_INV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&self) -> DAC_INV_R {
        DAC_INV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "1: to select CW generator as source to PDAC(1-2)_DAC\\[7:0\\] 0: to select register reg_pdac(1-2)_dac\\[7:0\\] as source to PDAC(1-2)_DAC\\[7:0\\]"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_CW_EN1` field.</div>"]
    #[inline(always)]
    pub fn dac_cw_en(&self, n: u8) -> DAC_CW_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_CW_EN_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "1: to select CW generator as source to PDAC(1-2)_DAC\\[7:0\\] 0: to select register reg_pdac(1-2)_dac\\[7:0\\] as source to PDAC(1-2)_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en_iter(&self) -> impl Iterator<Item = DAC_CW_EN_R> + '_ {
        (0..2).map(move |n| DAC_CW_EN_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\] 0: to select register reg_pdac1_dac\\[7:0\\] as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&self) -> DAC_CW_EN_R {
        DAC_CW_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\] 0: to select register reg_pdac2_dac\\[7:0\\] as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&self) -> DAC_CW_EN_R {
        DAC_CW_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DAC_CTRL2")
            .field("dac_dc1", &self.dac_dc1())
            .field("dac_dc2", &self.dac_dc2())
            .field("dac_scale1", &self.dac_scale1())
            .field("dac_scale2", &self.dac_scale2())
            .field("dac_inv1", &self.dac_inv1())
            .field("dac_inv2", &self.dac_inv2())
            .field("dac_cw_en1", &self.dac_cw_en1())
            .field("dac_cw_en2", &self.dac_cw_en2())
            .finish()
    }
}
impl W {
    #[doc = "DC offset for DAC(1-2) CW generator"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_DC1` field.</div>"]
    #[inline(always)]
    pub fn dac_dc(&mut self, n: u8) -> DAC_DC_W<'_, SAR_DAC_CTRL2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_DC_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&mut self) -> DAC_DC_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_DC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&mut self) -> DAC_DC_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_DC_W::new(self, 8)
    }
    #[doc = "00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_SCALE1` field.</div>"]
    #[inline(always)]
    pub fn dac_scale(&mut self, n: u8) -> DAC_SCALE_W<'_, SAR_DAC_CTRL2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_SCALE_W::new(self, n * 2 + 16)
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&mut self) -> DAC_SCALE_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_SCALE_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&mut self) -> DAC_SCALE_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_SCALE_W::new(self, 18)
    }
    #[doc = "00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_INV1` field.</div>"]
    #[inline(always)]
    pub fn dac_inv(&mut self, n: u8) -> DAC_INV_W<'_, SAR_DAC_CTRL2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_INV_W::new(self, n * 2 + 20)
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&mut self) -> DAC_INV_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_INV_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&mut self) -> DAC_INV_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_INV_W::new(self, 22)
    }
    #[doc = "1: to select CW generator as source to PDAC(1-2)_DAC\\[7:0\\] 0: to select register reg_pdac(1-2)_dac\\[7:0\\] as source to PDAC(1-2)_DAC\\[7:0\\]"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DAC_CW_EN1` field.</div>"]
    #[inline(always)]
    pub fn dac_cw_en(&mut self, n: u8) -> DAC_CW_EN_W<'_, SAR_DAC_CTRL2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DAC_CW_EN_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\] 0: to select register reg_pdac1_dac\\[7:0\\] as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&mut self) -> DAC_CW_EN_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_CW_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\] 0: to select register reg_pdac2_dac\\[7:0\\] as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&mut self) -> DAC_CW_EN_W<'_, SAR_DAC_CTRL2_SPEC> {
        DAC_CW_EN_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_dac_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_dac_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_DAC_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_dac_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_dac_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_DAC_CTRL2 to value 0x0300_0000"]
impl crate::Resettable for SAR_DAC_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0300_0000;
}
