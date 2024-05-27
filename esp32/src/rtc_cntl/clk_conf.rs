#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CK8M_DIV {
    #[doc = "0: DIV128"]
    Div128 = 0,
    #[doc = "1: DIV256"]
    Div256 = 1,
    #[doc = "2: DIV512"]
    Div512 = 2,
    #[doc = "3: DIV1024"]
    Div1024 = 3,
}
impl From<CK8M_DIV> for u8 {
    #[inline(always)]
    fn from(variant: CK8M_DIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CK8M_DIV {
    type Ux = u8;
}
impl crate::IsEnum for CK8M_DIV {}
#[doc = "Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub type CK8M_DIV_R = crate::FieldReader<CK8M_DIV>;
impl CK8M_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CK8M_DIV {
        match self.bits {
            0 => CK8M_DIV::Div128,
            1 => CK8M_DIV::Div256,
            2 => CK8M_DIV::Div512,
            3 => CK8M_DIV::Div1024,
            _ => unreachable!(),
        }
    }
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CK8M_DIV::Div128
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CK8M_DIV::Div256
    }
    #[doc = "DIV512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CK8M_DIV::Div512
    }
    #[doc = "DIV1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CK8M_DIV::Div1024
    }
}
#[doc = "Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
pub type CK8M_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CK8M_DIV, crate::Safe>;
impl<'a, REG> CK8M_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CK8M_DIV::Div128)
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(CK8M_DIV::Div256)
    }
    #[doc = "DIV512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(CK8M_DIV::Div512)
    }
    #[doc = "DIV1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(CK8M_DIV::Div1024)
    }
}
#[doc = "Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_R = crate::BitReader;
#[doc = "Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT"]
pub type ENB_CK8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_CK8M_DIV {
    #[doc = "0: CK8M_DIV_256"]
    Ck8mDiv256 = 0,
    #[doc = "1: CK8M"]
    Ck8m = 1,
}
impl From<ENB_CK8M_DIV> for bool {
    #[inline(always)]
    fn from(variant: ENB_CK8M_DIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub type ENB_CK8M_DIV_R = crate::BitReader<ENB_CK8M_DIV>;
impl ENB_CK8M_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENB_CK8M_DIV {
        match self.bits {
            false => ENB_CK8M_DIV::Ck8mDiv256,
            true => ENB_CK8M_DIV::Ck8m,
        }
    }
    #[doc = "CK8M_DIV_256"]
    #[inline(always)]
    pub fn is_ck8m_div_256(&self) -> bool {
        *self == ENB_CK8M_DIV::Ck8mDiv256
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == ENB_CK8M_DIV::Ck8m
    }
}
#[doc = "Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
pub type ENB_CK8M_DIV_W<'a, REG> = crate::BitWriter<'a, REG, ENB_CK8M_DIV>;
impl<'a, REG> ENB_CK8M_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK8M_DIV_256"]
    #[inline(always)]
    pub fn ck8m_div_256(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_CK8M_DIV::Ck8mDiv256)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_CK8M_DIV::Ck8m)
    }
}
#[doc = "Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_R = crate::BitReader;
#[doc = "Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
pub type DIG_XTAL32K_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_D256_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_R = crate::BitReader;
#[doc = "Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)"]
pub type DIG_CLK8M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DFREQ_FORCE` reader - "]
pub type CK8M_DFREQ_FORCE_R = crate::BitReader;
#[doc = "Field `CK8M_DFREQ_FORCE` writer - "]
pub type CK8M_DFREQ_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_R = crate::FieldReader;
#[doc = "Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1"]
pub type CK8M_DIV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep"]
pub type XTAL_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep"]
pub type CK8M_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_DFREQ` reader - CK8M_DFREQ"]
pub type CK8M_DFREQ_R = crate::FieldReader;
#[doc = "Field `CK8M_DFREQ` writer - CK8M_DFREQ"]
pub type CK8M_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CK8M_FORCE_PD` reader - CK8M force power down"]
pub type CK8M_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PD` writer - CK8M force power down"]
pub type CK8M_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK8M_FORCE_PU` reader - CK8M force power up"]
pub type CK8M_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CK8M_FORCE_PU` writer - CK8M force power up"]
pub type CK8M_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOC_CLK_SEL {
    #[doc = "0: XTAL"]
    Xtal = 0,
    #[doc = "1: PLL"]
    Pll = 1,
    #[doc = "2: CK8M"]
    Ck8m = 2,
    #[doc = "3: APLL"]
    Apll = 3,
}
impl From<SOC_CLK_SEL> for u8 {
    #[inline(always)]
    fn from(variant: SOC_CLK_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOC_CLK_SEL {
    type Ux = u8;
}
impl crate::IsEnum for SOC_CLK_SEL {}
#[doc = "Field `SOC_CLK_SEL` reader - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub type SOC_CLK_SEL_R = crate::FieldReader<SOC_CLK_SEL>;
impl SOC_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOC_CLK_SEL {
        match self.bits {
            0 => SOC_CLK_SEL::Xtal,
            1 => SOC_CLK_SEL::Pll,
            2 => SOC_CLK_SEL::Ck8m,
            3 => SOC_CLK_SEL::Apll,
            _ => unreachable!(),
        }
    }
    #[doc = "XTAL"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SOC_CLK_SEL::Xtal
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SOC_CLK_SEL::Pll
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == SOC_CLK_SEL::Ck8m
    }
    #[doc = "APLL"]
    #[inline(always)]
    pub fn is_apll(&self) -> bool {
        *self == SOC_CLK_SEL::Apll
    }
}
#[doc = "Field `SOC_CLK_SEL` writer - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
pub type SOC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SOC_CLK_SEL, crate::Safe>;
impl<'a, REG> SOC_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XTAL"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(SOC_CLK_SEL::Xtal)
    }
    #[doc = "PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SOC_CLK_SEL::Pll)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut crate::W<REG> {
        self.variant(SOC_CLK_SEL::Ck8m)
    }
    #[doc = "APLL"]
    #[inline(always)]
    pub fn apll(self) -> &'a mut crate::W<REG> {
        self.variant(SOC_CLK_SEL::Apll)
    }
}
#[doc = "fast_clk_rtc sel. 0: XTAL div 4 1: CK8M\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST_CLK_RTC_SEL {
    #[doc = "0: XTAL_DIV_4"]
    XtalDiv4 = 0,
    #[doc = "1: CK8M"]
    Ck8m = 1,
}
impl From<FAST_CLK_RTC_SEL> for bool {
    #[inline(always)]
    fn from(variant: FAST_CLK_RTC_SEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub type FAST_CLK_RTC_SEL_R = crate::BitReader<FAST_CLK_RTC_SEL>;
impl FAST_CLK_RTC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FAST_CLK_RTC_SEL {
        match self.bits {
            false => FAST_CLK_RTC_SEL::XtalDiv4,
            true => FAST_CLK_RTC_SEL::Ck8m,
        }
    }
    #[doc = "XTAL_DIV_4"]
    #[inline(always)]
    pub fn is_xtal_div_4(&self) -> bool {
        *self == FAST_CLK_RTC_SEL::XtalDiv4
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn is_ck8m(&self) -> bool {
        *self == FAST_CLK_RTC_SEL::Ck8m
    }
}
#[doc = "Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
pub type FAST_CLK_RTC_SEL_W<'a, REG> = crate::BitWriter<'a, REG, FAST_CLK_RTC_SEL>;
impl<'a, REG> FAST_CLK_RTC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "XTAL_DIV_4"]
    #[inline(always)]
    pub fn xtal_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_CLK_RTC_SEL::XtalDiv4)
    }
    #[doc = "CK8M"]
    #[inline(always)]
    pub fn ck8m(self) -> &'a mut crate::W<REG> {
        self.variant(FAST_CLK_RTC_SEL::Ck8m)
    }
}
#[doc = "slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANA_CLK_RTC_SEL {
    #[doc = "0: SLOW_CK"]
    SlowCk = 0,
    #[doc = "1: CK_XTAL_32K"]
    CkXtal32k = 1,
    #[doc = "2: CK8M_D256_OUT"]
    Ck8mD256Out = 2,
}
impl From<ANA_CLK_RTC_SEL> for u8 {
    #[inline(always)]
    fn from(variant: ANA_CLK_RTC_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANA_CLK_RTC_SEL {
    type Ux = u8;
}
impl crate::IsEnum for ANA_CLK_RTC_SEL {}
#[doc = "Field `ANA_CLK_RTC_SEL` reader - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader<ANA_CLK_RTC_SEL>;
impl ANA_CLK_RTC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ANA_CLK_RTC_SEL> {
        match self.bits {
            0 => Some(ANA_CLK_RTC_SEL::SlowCk),
            1 => Some(ANA_CLK_RTC_SEL::CkXtal32k),
            2 => Some(ANA_CLK_RTC_SEL::Ck8mD256Out),
            _ => None,
        }
    }
    #[doc = "SLOW_CK"]
    #[inline(always)]
    pub fn is_slow_ck(&self) -> bool {
        *self == ANA_CLK_RTC_SEL::SlowCk
    }
    #[doc = "CK_XTAL_32K"]
    #[inline(always)]
    pub fn is_ck_xtal_32k(&self) -> bool {
        *self == ANA_CLK_RTC_SEL::CkXtal32k
    }
    #[doc = "CK8M_D256_OUT"]
    #[inline(always)]
    pub fn is_ck8m_d256_out(&self) -> bool {
        *self == ANA_CLK_RTC_SEL::Ck8mD256Out
    }
}
#[doc = "Field `ANA_CLK_RTC_SEL` writer - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
pub type ANA_CLK_RTC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANA_CLK_RTC_SEL>;
impl<'a, REG> ANA_CLK_RTC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SLOW_CK"]
    #[inline(always)]
    pub fn slow_ck(self) -> &'a mut crate::W<REG> {
        self.variant(ANA_CLK_RTC_SEL::SlowCk)
    }
    #[doc = "CK_XTAL_32K"]
    #[inline(always)]
    pub fn ck_xtal_32k(self) -> &'a mut crate::W<REG> {
        self.variant(ANA_CLK_RTC_SEL::CkXtal32k)
    }
    #[doc = "CK8M_D256_OUT"]
    #[inline(always)]
    pub fn ck8m_d256_out(self) -> &'a mut crate::W<REG> {
        self.variant(ANA_CLK_RTC_SEL::Ck8mD256Out)
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
            .field("ck8m_div", &self.ck8m_div())
            .field("enb_ck8m", &self.enb_ck8m())
            .field("enb_ck8m_div", &self.enb_ck8m_div())
            .field("dig_xtal32k_en", &self.dig_xtal32k_en())
            .field("dig_clk8m_d256_en", &self.dig_clk8m_d256_en())
            .field("dig_clk8m_en", &self.dig_clk8m_en())
            .field("ck8m_dfreq_force", &self.ck8m_dfreq_force())
            .field("ck8m_div_sel", &self.ck8m_div_sel())
            .field("xtal_force_nogating", &self.xtal_force_nogating())
            .field("ck8m_force_nogating", &self.ck8m_force_nogating())
            .field("ck8m_dfreq", &self.ck8m_dfreq())
            .field("ck8m_force_pd", &self.ck8m_force_pd())
            .field("ck8m_force_pu", &self.ck8m_force_pu())
            .field("soc_clk_sel", &self.soc_clk_sel())
            .field("fast_clk_rtc_sel", &self.fast_clk_rtc_sel())
            .field("ana_clk_rtc_sel", &self.ana_clk_rtc_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W<CLK_CONF_SPEC> {
        CK8M_DIV_W::new(self, 4)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W<CLK_CONF_SPEC> {
        ENB_CK8M_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W<CLK_CONF_SPEC> {
        ENB_CK8M_DIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W<CLK_CONF_SPEC> {
        DIG_XTAL32K_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_D256_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq_force(&mut self) -> CK8M_DFREQ_FORCE_W<CLK_CONF_SPEC> {
        CK8M_DFREQ_FORCE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W<CLK_CONF_SPEC> {
        CK8M_DIV_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        XTAL_FORCE_NOGATING_W::new(self, 15)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        CK8M_FORCE_NOGATING_W::new(self, 16)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W<CLK_CONF_SPEC> {
        CK8M_DFREQ_W::new(self, 17)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PD_W::new(self, 25)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PU_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    #[must_use]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W<CLK_CONF_SPEC> {
        SOC_CLK_SEL_W::new(self, 27)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        FAST_CLK_RTC_SEL_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        ANA_CLK_RTC_SEL_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x2210"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x2210;
}
