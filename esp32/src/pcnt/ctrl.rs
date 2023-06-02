#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_RST_U0` reader - Set this bit to clear unit0's counter."]
pub type CNT_RST_U0_R = crate::BitReader;
#[doc = "Field `CNT_RST_U0` writer - Set this bit to clear unit0's counter."]
pub type CNT_RST_U0_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U0` reader - Set this bit to pause unit0's counter."]
pub type CNT_PAUSE_U0_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U0` writer - Set this bit to pause unit0's counter."]
pub type CNT_PAUSE_U0_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U1` reader - Set this bit to clear unit1's counter."]
pub type CNT_RST_U1_R = crate::BitReader;
#[doc = "Field `CNT_RST_U1` writer - Set this bit to clear unit1's counter."]
pub type CNT_RST_U1_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U1` reader - Set this bit to pause unit1's counter."]
pub type CNT_PAUSE_U1_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U1` writer - Set this bit to pause unit1's counter."]
pub type CNT_PAUSE_U1_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U2` reader - Set this bit to clear unit2's counter."]
pub type CNT_RST_U2_R = crate::BitReader;
#[doc = "Field `CNT_RST_U2` writer - Set this bit to clear unit2's counter."]
pub type CNT_RST_U2_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U2` reader - Set this bit to pause unit2's counter."]
pub type CNT_PAUSE_U2_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U2` writer - Set this bit to pause unit2's counter."]
pub type CNT_PAUSE_U2_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U3` reader - Set this bit to clear unit3's counter."]
pub type CNT_RST_U3_R = crate::BitReader;
#[doc = "Field `CNT_RST_U3` writer - Set this bit to clear unit3's counter."]
pub type CNT_RST_U3_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U3` reader - Set this bit to pause unit3's counter."]
pub type CNT_PAUSE_U3_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U3` writer - Set this bit to pause unit3's counter."]
pub type CNT_PAUSE_U3_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U4` reader - Set this bit to clear unit4's counter."]
pub type CNT_RST_U4_R = crate::BitReader;
#[doc = "Field `CNT_RST_U4` writer - Set this bit to clear unit4's counter."]
pub type CNT_RST_U4_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U4` reader - Set this bit to pause unit4's counter."]
pub type CNT_PAUSE_U4_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U4` writer - Set this bit to pause unit4's counter."]
pub type CNT_PAUSE_U4_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U5` reader - Set this bit to clear unit5's counter."]
pub type CNT_RST_U5_R = crate::BitReader;
#[doc = "Field `CNT_RST_U5` writer - Set this bit to clear unit5's counter."]
pub type CNT_RST_U5_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U5` reader - Set this bit to pause unit5's counter."]
pub type CNT_PAUSE_U5_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U5` writer - Set this bit to pause unit5's counter."]
pub type CNT_PAUSE_U5_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U6` reader - Set this bit to clear unit6's counter."]
pub type CNT_RST_U6_R = crate::BitReader;
#[doc = "Field `CNT_RST_U6` writer - Set this bit to clear unit6's counter."]
pub type CNT_RST_U6_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U6` reader - Set this bit to pause unit6's counter."]
pub type CNT_PAUSE_U6_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U6` writer - Set this bit to pause unit6's counter."]
pub type CNT_PAUSE_U6_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_RST_U7` reader - Set this bit to clear unit7's counter."]
pub type CNT_RST_U7_R = crate::BitReader;
#[doc = "Field `CNT_RST_U7` writer - Set this bit to clear unit7's counter."]
pub type CNT_RST_U7_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CNT_PAUSE_U7` reader - Set this bit to pause unit7's counter."]
pub type CNT_PAUSE_U7_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U7` writer - Set this bit to pause unit7's counter."]
pub type CNT_PAUSE_U7_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    pub fn cnt_rst_u0(&self) -> CNT_RST_U0_R {
        CNT_RST_U0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&self) -> CNT_PAUSE_U0_R {
        CNT_PAUSE_U0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    pub fn cnt_rst_u1(&self) -> CNT_RST_U1_R {
        CNT_RST_U1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&self) -> CNT_PAUSE_U1_R {
        CNT_PAUSE_U1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    pub fn cnt_rst_u2(&self) -> CNT_RST_U2_R {
        CNT_RST_U2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&self) -> CNT_PAUSE_U2_R {
        CNT_PAUSE_U2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    pub fn cnt_rst_u3(&self) -> CNT_RST_U3_R {
        CNT_RST_U3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&self) -> CNT_PAUSE_U3_R {
        CNT_PAUSE_U3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to clear unit4's counter."]
    #[inline(always)]
    pub fn cnt_rst_u4(&self) -> CNT_RST_U4_R {
        CNT_RST_U4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to pause unit4's counter."]
    #[inline(always)]
    pub fn cnt_pause_u4(&self) -> CNT_PAUSE_U4_R {
        CNT_PAUSE_U4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to clear unit5's counter."]
    #[inline(always)]
    pub fn cnt_rst_u5(&self) -> CNT_RST_U5_R {
        CNT_RST_U5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to pause unit5's counter."]
    #[inline(always)]
    pub fn cnt_pause_u5(&self) -> CNT_PAUSE_U5_R {
        CNT_PAUSE_U5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to clear unit6's counter."]
    #[inline(always)]
    pub fn cnt_rst_u6(&self) -> CNT_RST_U6_R {
        CNT_RST_U6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to pause unit6's counter."]
    #[inline(always)]
    pub fn cnt_pause_u6(&self) -> CNT_PAUSE_U6_R {
        CNT_PAUSE_U6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to clear unit7's counter."]
    #[inline(always)]
    pub fn cnt_rst_u7(&self) -> CNT_RST_U7_R {
        CNT_RST_U7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to pause unit7's counter."]
    #[inline(always)]
    pub fn cnt_pause_u7(&self) -> CNT_PAUSE_U7_R {
        CNT_PAUSE_U7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("cnt_rst_u0", &format_args!("{}", self.cnt_rst_u0().bit()))
            .field(
                "cnt_pause_u0",
                &format_args!("{}", self.cnt_pause_u0().bit()),
            )
            .field("cnt_rst_u1", &format_args!("{}", self.cnt_rst_u1().bit()))
            .field(
                "cnt_pause_u1",
                &format_args!("{}", self.cnt_pause_u1().bit()),
            )
            .field("cnt_rst_u2", &format_args!("{}", self.cnt_rst_u2().bit()))
            .field(
                "cnt_pause_u2",
                &format_args!("{}", self.cnt_pause_u2().bit()),
            )
            .field("cnt_rst_u3", &format_args!("{}", self.cnt_rst_u3().bit()))
            .field(
                "cnt_pause_u3",
                &format_args!("{}", self.cnt_pause_u3().bit()),
            )
            .field("cnt_rst_u4", &format_args!("{}", self.cnt_rst_u4().bit()))
            .field(
                "cnt_pause_u4",
                &format_args!("{}", self.cnt_pause_u4().bit()),
            )
            .field("cnt_rst_u5", &format_args!("{}", self.cnt_rst_u5().bit()))
            .field(
                "cnt_pause_u5",
                &format_args!("{}", self.cnt_pause_u5().bit()),
            )
            .field("cnt_rst_u6", &format_args!("{}", self.cnt_rst_u6().bit()))
            .field(
                "cnt_pause_u6",
                &format_args!("{}", self.cnt_pause_u6().bit()),
            )
            .field("cnt_rst_u7", &format_args!("{}", self.cnt_rst_u7().bit()))
            .field(
                "cnt_pause_u7",
                &format_args!("{}", self.cnt_pause_u7().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u0(&mut self) -> CNT_RST_U0_W<0> {
        CNT_RST_U0_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u0(&mut self) -> CNT_PAUSE_U0_W<1> {
        CNT_PAUSE_U0_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u1(&mut self) -> CNT_RST_U1_W<2> {
        CNT_RST_U1_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u1(&mut self) -> CNT_PAUSE_U1_W<3> {
        CNT_PAUSE_U1_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u2(&mut self) -> CNT_RST_U2_W<4> {
        CNT_RST_U2_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u2(&mut self) -> CNT_PAUSE_U2_W<5> {
        CNT_PAUSE_U2_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u3(&mut self) -> CNT_RST_U3_W<6> {
        CNT_RST_U3_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u3(&mut self) -> CNT_PAUSE_U3_W<7> {
        CNT_PAUSE_U3_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear unit4's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u4(&mut self) -> CNT_RST_U4_W<8> {
        CNT_RST_U4_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to pause unit4's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u4(&mut self) -> CNT_PAUSE_U4_W<9> {
        CNT_PAUSE_U4_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear unit5's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u5(&mut self) -> CNT_RST_U5_W<10> {
        CNT_RST_U5_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to pause unit5's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u5(&mut self) -> CNT_PAUSE_U5_W<11> {
        CNT_PAUSE_U5_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear unit6's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u6(&mut self) -> CNT_RST_U6_W<12> {
        CNT_RST_U6_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to pause unit6's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u6(&mut self) -> CNT_PAUSE_U6_W<13> {
        CNT_PAUSE_U6_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear unit7's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst_u7(&mut self) -> CNT_RST_U7_W<14> {
        CNT_RST_U7_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to pause unit7's counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_pause_u7(&mut self) -> CNT_PAUSE_U7_W<15> {
        CNT_PAUSE_U7_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<16> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x5555"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x5555;
}
