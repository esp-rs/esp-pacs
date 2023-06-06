#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub type CK8M_DIV_R = crate::FieldReader<CK8M_DIV_A>;
#[doc = "CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CK8M_DIV_A {
    #[doc = "0: DIV128"]
    DIV128 = 0,
    #[doc = "1: DIV256"]
    DIV256 = 1,
    #[doc = "2: DIV512"]
    DIV512 = 2,
    #[doc = "3: DIV1024"]
    DIV1024 = 3,
}
impl From<CK8M_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CK8M_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CK8M_DIV_A {
    type Ux = u8;
}
impl CK8M_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK8M_DIV_A {
        match self.bits {
            0 => CK8M_DIV_A::DIV128,
            1 => CK8M_DIV_A::DIV256,
            2 => CK8M_DIV_A::DIV512,
            3 => CK8M_DIV_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CK8M_DIV_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CK8M_DIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CK8M_DIV_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CK8M_DIV_A::DIV1024
    }
}
#[doc = "Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub type CK8M_DIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CLK_CONF_SPEC, 2, O, CK8M_DIV_A>;
impl<'a, const O: u8> CK8M_DIV_W<'a, O> {
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV128)
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV256)
    }
    #[doc = "DIV512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV512)
    }
    #[doc = "DIV1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CK8M_DIV_A::DIV1024)
    }
}
#[doc = "Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_R = crate::BitReader;
#[doc = "Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub type ENB_CK8M_DIV_R = crate::BitReader<ENB_CK8M_DIV_A>;
#[doc = "1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_CK8M_DIV_A {
    #[doc = "0: CK8M_DIV_256"]
    CK8M_DIV_256 = 0,
    #[doc = "1: CK8M"]
    CK8M = 1,
}
impl From<ENB_CK8M_DIV_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_CK8M_DIV_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_CK8M_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_CK8M_DIV_A {
        match self.bits {
            false => ENB_CK8M_DIV_A::CK8M_DIV_256,
            true => ENB_CK8M_DIV_A::CK8M,
        }
    }
    #[doc = "Checks if the value of the field is `CK8M_DIV_256`"]
    #[inline(always)]
    pub fn is_ck8m_div_256(&self) -> bool {
        *self == ENB_CK8M_DIV_A::CK8M_DIV_256
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == ENB_CK8M_DIV_A::CK8M
    }
}
#[doc = "Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub type ENB_CK8M_DIV_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O, ENB_CK8M_DIV_A>;
impl<'a, const O: u8> ENB_CK8M_DIV_W<'a, O> {
    #[doc = "CK8M_DIV_256"]
    #[inline(always)]
    pub fn ck8m_div_256(self) -> &'a mut W {
        self.variant(ENB_CK8M_DIV_A::CK8M_DIV_256)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(ENB_CK8M_DIV_A::CK8M)
    }
}
#[doc = "Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_R = crate::BitReader;
#[doc = "Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DFREQ_FORCE` reader - "]
pub type CK8M_DFREQ_FORCE_R = crate::BitReader;
#[doc = "Field `CK8M_DFREQ_FORCE` writer - "]
pub type CK8M_DFREQ_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 3, O>;
#[doc = "Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub type CK8M_DFREQ_R = crate::FieldReader;
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub type CK8M_DFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_SPEC, 8, O>;
#[doc = "Field `CK8M_FORCE_PD` reader - CK8M force power down"]
pub type CK8M_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PD` writer - CK8M force power down"]
pub type CK8M_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `CK8M_FORCE_PU` reader - CK8M force power up"]
pub type CK8M_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PU` writer - CK8M force power up"]
pub type CK8M_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF_SPEC, O>;
#[doc = "Field `SOC_CLK_SEL` reader - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub type SOC_CLK_SEL_R = crate::FieldReader<SOC_CLK_SEL_A>;
#[doc = "SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOC_CLK_SEL_A {
    #[doc = "0: XTAL"]
    XTAL = 0,
    #[doc = "1: PLL"]
    PLL = 1,
    #[doc = "2: CK8M"]
    CK8M = 2,
    #[doc = "3: APLL"]
    APLL = 3,
}
impl From<SOC_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOC_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOC_CLK_SEL_A {
    type Ux = u8;
}
impl SOC_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOC_CLK_SEL_A {
        match self.bits {
            0 => SOC_CLK_SEL_A::XTAL,
            1 => SOC_CLK_SEL_A::PLL,
            2 => SOC_CLK_SEL_A::CK8M,
            3 => SOC_CLK_SEL_A::APLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SOC_CLK_SEL_A::XTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SOC_CLK_SEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == SOC_CLK_SEL_A::CK8M
    }
    #[doc = "Checks if the value of the field is `APLL`"]
    #[inline(always)]
    pub fn is_apll(&self) -> bool {
        *self == SOC_CLK_SEL_A::APLL
    }
}
#[doc = "Field `SOC_CLK_SEL` writer - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub type SOC_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CLK_CONF_SPEC, 2, O, SOC_CLK_SEL_A>;
impl<'a, const O: u8> SOC_CLK_SEL_W<'a, O> {
    #[doc = "XTAL"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::XTAL)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::PLL)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::CK8M)
    }
    #[doc = "APLL"]
    #[inline(always)]
    pub fn apll(self) -> &'a mut W {
        self.variant(SOC_CLK_SEL_A::APLL)
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader<FAST_CLK_RTC_SEL_A>;
#[doc = "fast_clk_rtc sel. 0: XTAL div 4 1: CK8M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST_CLK_RTC_SEL_A {
    #[doc = "0: XTAL_DIV_4"]
    XTAL_DIV_4 = 0,
    #[doc = "1: CK8M"]
    CK8M = 1,
}
impl From<FAST_CLK_RTC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_CLK_RTC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FAST_CLK_RTC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_CLK_RTC_SEL_A {
        match self.bits {
            false => FAST_CLK_RTC_SEL_A::XTAL_DIV_4,
            true => FAST_CLK_RTC_SEL_A::CK8M,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV_4`"]
    #[inline(always)]
    pub fn is_xtal_div_4(&self) -> bool {
        *self == FAST_CLK_RTC_SEL_A::XTAL_DIV_4
    }
    #[doc = "Checks if the value of the field is `CK8M`"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == FAST_CLK_RTC_SEL_A::CK8M
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub type FAST_CLK_RTC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, CLK_CONF_SPEC, O, FAST_CLK_RTC_SEL_A>;
impl<'a, const O: u8> FAST_CLK_RTC_SEL_W<'a, O> {
    #[doc = "XTAL_DIV_4"]
    #[inline(always)]
    pub fn xtal_div_4(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::XTAL_DIV_4)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut W {
        self.variant(FAST_CLK_RTC_SEL_A::CK8M)
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` reader - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader<ANA_CLK_RTC_SEL_A>;
#[doc = "slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANA_CLK_RTC_SEL_A {
    #[doc = "0: SLOW_CK"]
    SLOW_CK = 0,
    #[doc = "1: CK_XTAL_32K"]
    CK_XTAL_32K = 1,
    #[doc = "2: CK8M_D256_OUT"]
    CK8M_D256_OUT = 2,
}
impl From<ANA_CLK_RTC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ANA_CLK_RTC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANA_CLK_RTC_SEL_A {
    type Ux = u8;
}
impl ANA_CLK_RTC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANA_CLK_RTC_SEL_A> {
        match self.bits {
            0 => Some(ANA_CLK_RTC_SEL_A::SLOW_CK),
            1 => Some(ANA_CLK_RTC_SEL_A::CK_XTAL_32K),
            2 => Some(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CK`"]
    #[inline(always)]
    pub fn is_slow_ck(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::SLOW_CK
    }
    #[doc = "Checks if the value of the field is `CK_XTAL_32K`"]
    #[inline(always)]
    pub fn is_ck_xtal_32k(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::CK_XTAL_32K
    }
    #[doc = "Checks if the value of the field is `CK8M_D256_OUT`"]
    #[inline(always)]
    pub fn is_ck8m_d256_out(&self) -> bool {
        *self == ANA_CLK_RTC_SEL_A::CK8M_D256_OUT
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` writer - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub type ANA_CLK_RTC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_SPEC, 2, O, ANA_CLK_RTC_SEL_A>;
impl<'a, const O: u8> ANA_CLK_RTC_SEL_W<'a, O> {
    #[doc = "SLOW_CK"]
    #[inline(always)]
    pub fn slow_ck(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::SLOW_CK)
    }
    #[doc = "CK_XTAL_32K"]
    #[inline(always)]
    pub fn ck_xtal_32k(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK_XTAL_32K)
    }
    #[doc = "CK8M_D256_OUT"]
    #[inline(always)]
    pub fn ck8m_d256_out(self) -> &'a mut W {
        self.variant(ANA_CLK_RTC_SEL_A::CK8M_D256_OUT)
    }
}
impl R {
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ck8m_dfreq_force(&self) -> CK8M_DFREQ_FORCE_R {
        CK8M_DFREQ_FORCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    pub fn soc_clk_sel(&self) -> SOC_CLK_SEL_R {
        SOC_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("ck8m_div", &format_args!("{}", self.ck8m_div().bits()))
            .field("enb_ck8m", &format_args!("{}", self.enb_ck8m().bit()))
            .field(
                "enb_ck8m_div",
                &format_args!("{}", self.enb_ck8m_div().bit()),
            )
            .field(
                "dig_xtal32k_en",
                &format_args!("{}", self.dig_xtal32k_en().bit()),
            )
            .field(
                "dig_clk8m_d256_en",
                &format_args!("{}", self.dig_clk8m_d256_en().bit()),
            )
            .field(
                "dig_clk8m_en",
                &format_args!("{}", self.dig_clk8m_en().bit()),
            )
            .field(
                "ck8m_dfreq_force",
                &format_args!("{}", self.ck8m_dfreq_force().bit()),
            )
            .field(
                "ck8m_div_sel",
                &format_args!("{}", self.ck8m_div_sel().bits()),
            )
            .field(
                "xtal_force_nogating",
                &format_args!("{}", self.xtal_force_nogating().bit()),
            )
            .field(
                "ck8m_force_nogating",
                &format_args!("{}", self.ck8m_force_nogating().bit()),
            )
            .field("ck8m_dfreq", &format_args!("{}", self.ck8m_dfreq().bits()))
            .field(
                "ck8m_force_pd",
                &format_args!("{}", self.ck8m_force_pd().bit()),
            )
            .field(
                "ck8m_force_pu",
                &format_args!("{}", self.ck8m_force_pu().bit()),
            )
            .field(
                "soc_clk_sel",
                &format_args!("{}", self.soc_clk_sel().bits()),
            )
            .field(
                "fast_clk_rtc_sel",
                &format_args!("{}", self.fast_clk_rtc_sel().bit()),
            )
            .field(
                "ana_clk_rtc_sel",
                &format_args!("{}", self.ana_clk_rtc_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W<4> {
        CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W<6> {
        ENB_CK8M_W::new(self)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W<7> {
        ENB_CK8M_DIV_W::new(self)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W<8> {
        DIG_XTAL32K_EN_W::new(self)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W<9> {
        DIG_CLK8M_D256_EN_W::new(self)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W<10> {
        DIG_CLK8M_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq_force(&mut self) -> CK8M_DFREQ_FORCE_W<11> {
        CK8M_DFREQ_FORCE_W::new(self)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W<12> {
        CK8M_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W<15> {
        XTAL_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W<16> {
        CK8M_FORCE_NOGATING_W::new(self)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W<17> {
        CK8M_DFREQ_W::new(self)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W<25> {
        CK8M_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W<26> {
        CK8M_FORCE_PU_W::new(self)
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    #[must_use]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W<27> {
        SOC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<29> {
        FAST_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W<30> {
        ANA_CLK_RTC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x2210"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x2210;
}
