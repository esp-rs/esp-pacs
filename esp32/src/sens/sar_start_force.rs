#[doc = "Register `SAR_START_FORCE` reader"]
pub type R = crate::R<SAR_START_FORCE_SPEC>;
#[doc = "Register `SAR_START_FORCE` writer"]
pub type W = crate::W<SAR_START_FORCE_SPEC>;
#[doc = "Field `SAR1_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_R = crate::FieldReader;
#[doc = "Field `SAR1_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_R = crate::FieldReader;
#[doc = "Field `SAR2_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_R = crate::BitReader;
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARCLK_EN` reader - "]
pub type SARCLK_EN_R = crate::BitReader;
#[doc = "Field `SARCLK_EN` writer - "]
pub type SARCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC_INIT` reader - initialized PC for ULP-coprocessor"]
pub type PC_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `PC_INIT` writer - initialized PC for ULP-coprocessor"]
pub type PC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SAR2_STOP` reader - stop SAR ADC2 conversion"]
pub type SAR2_STOP_R = crate::BitReader;
#[doc = "Field `SAR2_STOP` writer - stop SAR ADC2 conversion"]
pub type SAR2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_STOP` reader - stop SAR ADC1 conversion"]
pub type SAR1_STOP_R = crate::BitReader;
#[doc = "Field `SAR1_STOP` writer - stop SAR ADC1 conversion"]
pub type SAR1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PWDET_EN` reader - N/A"]
pub type SAR2_PWDET_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PWDET_EN` writer - N/A"]
pub type SAR2_PWDET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&self) -> SAR1_BIT_WIDTH_R {
        SAR1_BIT_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&self) -> SAR2_BIT_WIDTH_R {
        SAR2_BIT_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&self) -> PC_INIT_R {
        PC_INIT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&self) -> SAR2_STOP_R {
        SAR2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&self) -> SAR1_STOP_R {
        SAR1_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&self) -> SAR2_PWDET_EN_R {
        SAR2_PWDET_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_START_FORCE")
            .field("sar1_bit_width", &self.sar1_bit_width())
            .field("sar2_bit_width", &self.sar2_bit_width())
            .field("sar2_en_test", &self.sar2_en_test())
            .field("sar2_pwdet_cct", &self.sar2_pwdet_cct())
            .field("ulp_cp_force_start_top", &self.ulp_cp_force_start_top())
            .field("ulp_cp_start_top", &self.ulp_cp_start_top())
            .field("sarclk_en", &self.sarclk_en())
            .field("pc_init", &self.pc_init())
            .field("sar2_stop", &self.sar2_stop())
            .field("sar1_stop", &self.sar1_stop())
            .field("sar2_pwdet_en", &self.sar2_pwdet_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&mut self) -> SAR1_BIT_WIDTH_W<SAR_START_FORCE_SPEC> {
        SAR1_BIT_WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&mut self) -> SAR2_BIT_WIDTH_W<SAR_START_FORCE_SPEC> {
        SAR2_BIT_WIDTH_W::new(self, 2)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W<SAR_START_FORCE_SPEC> {
        SAR2_EN_TEST_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<SAR_START_FORCE_SPEC> {
        SAR2_PWDET_CCT_W::new(self, 5)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W<SAR_START_FORCE_SPEC> {
        ULP_CP_FORCE_START_TOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W<SAR_START_FORCE_SPEC> {
        ULP_CP_START_TOP_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W<SAR_START_FORCE_SPEC> {
        SARCLK_EN_W::new(self, 10)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&mut self) -> PC_INIT_W<SAR_START_FORCE_SPEC> {
        PC_INIT_W::new(self, 11)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&mut self) -> SAR2_STOP_W<SAR_START_FORCE_SPEC> {
        SAR2_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&mut self) -> SAR1_STOP_W<SAR_START_FORCE_SPEC> {
        SAR1_STOP_W::new(self, 23)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&mut self) -> SAR2_PWDET_EN_W<SAR_START_FORCE_SPEC> {
        SAR2_PWDET_EN_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_start_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_start_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_START_FORCE_SPEC;
impl crate::RegisterSpec for SAR_START_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_start_force::R`](R) reader structure"]
impl crate::Readable for SAR_START_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_start_force::W`](W) writer structure"]
impl crate::Writable for SAR_START_FORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_START_FORCE to value 0x0f"]
impl crate::Resettable for SAR_START_FORCE_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
