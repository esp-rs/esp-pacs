#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CNT_RST_U(0-3)` reader - Set this bit to clear unit%s's counter."]
pub type CNT_RST_U_R = crate::BitReader;
#[doc = "Field `CNT_RST_U(0-3)` writer - Set this bit to clear unit%s's counter."]
pub type CNT_RST_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_PAUSE_U(0-3)` reader - Set this bit to pause unit%s's counter."]
pub type CNT_PAUSE_U_R = crate::BitReader;
#[doc = "Field `CNT_PAUSE_U(0-3)` writer - Set this bit to pause unit%s's counter."]
pub type CNT_PAUSE_U_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DALTA_CHANGE_EN_U0` reader - Configures this bit to enable unit 0's step comparator."]
pub type DALTA_CHANGE_EN_U0_R = crate::BitReader;
#[doc = "Field `DALTA_CHANGE_EN_U0` writer - Configures this bit to enable unit 0's step comparator."]
pub type DALTA_CHANGE_EN_U0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DALTA_CHANGE_EN_U1` reader - Configures this bit to enable unit 1's step comparator."]
pub type DALTA_CHANGE_EN_U1_R = crate::BitReader;
#[doc = "Field `DALTA_CHANGE_EN_U1` writer - Configures this bit to enable unit 1's step comparator."]
pub type DALTA_CHANGE_EN_U1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DALTA_CHANGE_EN_U2` reader - Configures this bit to enable unit 2's step comparator."]
pub type DALTA_CHANGE_EN_U2_R = crate::BitReader;
#[doc = "Field `DALTA_CHANGE_EN_U2` writer - Configures this bit to enable unit 2's step comparator."]
pub type DALTA_CHANGE_EN_U2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DALTA_CHANGE_EN_U3` reader - Configures this bit to enable unit 3's step comparator."]
pub type DALTA_CHANGE_EN_U3_R = crate::BitReader;
#[doc = "Field `DALTA_CHANGE_EN_U3` writer - Configures this bit to enable unit 3's step comparator."]
pub type DALTA_CHANGE_EN_U3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to clear unit(0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_RST_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_rst_u(&self, n: u8) -> CNT_RST_U_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_RST_U_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to clear unit(0-3)'s counter."]
    #[inline(always)]
    pub fn cnt_rst_u_iter(&self) -> impl Iterator<Item = CNT_RST_U_R> + '_ {
        (0..4).map(move |n| CNT_RST_U_R::new(((self.bits >> (n * 2)) & 1) != 0))
    }
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    pub fn cnt_rst_u0(&self) -> CNT_RST_U_R {
        CNT_RST_U_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    pub fn cnt_rst_u1(&self) -> CNT_RST_U_R {
        CNT_RST_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    pub fn cnt_rst_u2(&self) -> CNT_RST_U_R {
        CNT_RST_U_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    pub fn cnt_rst_u3(&self) -> CNT_RST_U_R {
        CNT_RST_U_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Set this bit to pause unit(0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_PAUSE_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_pause_u(&self, n: u8) -> CNT_PAUSE_U_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_PAUSE_U_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to pause unit(0-3)'s counter."]
    #[inline(always)]
    pub fn cnt_pause_u_iter(&self) -> impl Iterator<Item = CNT_PAUSE_U_R> + '_ {
        (0..4).map(move |n| CNT_PAUSE_U_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&self) -> CNT_PAUSE_U_R {
        CNT_PAUSE_U_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&self) -> CNT_PAUSE_U_R {
        CNT_PAUSE_U_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&self) -> CNT_PAUSE_U_R {
        CNT_PAUSE_U_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&self) -> CNT_PAUSE_U_R {
        CNT_PAUSE_U_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures this bit to enable unit 0's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u0(&self) -> DALTA_CHANGE_EN_U0_R {
        DALTA_CHANGE_EN_U0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures this bit to enable unit 1's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u1(&self) -> DALTA_CHANGE_EN_U1_R {
        DALTA_CHANGE_EN_U1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures this bit to enable unit 2's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u2(&self) -> DALTA_CHANGE_EN_U2_R {
        DALTA_CHANGE_EN_U2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures this bit to enable unit 3's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u3(&self) -> DALTA_CHANGE_EN_U3_R {
        DALTA_CHANGE_EN_U3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("cnt_rst_u0", &self.cnt_rst_u0())
            .field("cnt_rst_u1", &self.cnt_rst_u1())
            .field("cnt_rst_u2", &self.cnt_rst_u2())
            .field("cnt_rst_u3", &self.cnt_rst_u3())
            .field("cnt_pause_u0", &self.cnt_pause_u0())
            .field("cnt_pause_u1", &self.cnt_pause_u1())
            .field("cnt_pause_u2", &self.cnt_pause_u2())
            .field("cnt_pause_u3", &self.cnt_pause_u3())
            .field("dalta_change_en_u0", &self.dalta_change_en_u0())
            .field("dalta_change_en_u1", &self.dalta_change_en_u1())
            .field("dalta_change_en_u2", &self.dalta_change_en_u2())
            .field("dalta_change_en_u3", &self.dalta_change_en_u3())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Set this bit to clear unit(0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_RST_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_rst_u(&mut self, n: u8) -> CNT_RST_U_W<'_, CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_RST_U_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Set this bit to clear unit0's counter."]
    #[inline(always)]
    pub fn cnt_rst_u0(&mut self) -> CNT_RST_U_W<'_, CTRL_SPEC> {
        CNT_RST_U_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to clear unit1's counter."]
    #[inline(always)]
    pub fn cnt_rst_u1(&mut self) -> CNT_RST_U_W<'_, CTRL_SPEC> {
        CNT_RST_U_W::new(self, 2)
    }
    #[doc = "Bit 4 - Set this bit to clear unit2's counter."]
    #[inline(always)]
    pub fn cnt_rst_u2(&mut self) -> CNT_RST_U_W<'_, CTRL_SPEC> {
        CNT_RST_U_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to clear unit3's counter."]
    #[inline(always)]
    pub fn cnt_rst_u3(&mut self) -> CNT_RST_U_W<'_, CTRL_SPEC> {
        CNT_RST_U_W::new(self, 6)
    }
    #[doc = "Set this bit to pause unit(0-3)'s counter."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CNT_PAUSE_U0` field.</div>"]
    #[inline(always)]
    pub fn cnt_pause_u(&mut self, n: u8) -> CNT_PAUSE_U_W<'_, CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CNT_PAUSE_U_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Set this bit to pause unit0's counter."]
    #[inline(always)]
    pub fn cnt_pause_u0(&mut self) -> CNT_PAUSE_U_W<'_, CTRL_SPEC> {
        CNT_PAUSE_U_W::new(self, 1)
    }
    #[doc = "Bit 3 - Set this bit to pause unit1's counter."]
    #[inline(always)]
    pub fn cnt_pause_u1(&mut self) -> CNT_PAUSE_U_W<'_, CTRL_SPEC> {
        CNT_PAUSE_U_W::new(self, 3)
    }
    #[doc = "Bit 5 - Set this bit to pause unit2's counter."]
    #[inline(always)]
    pub fn cnt_pause_u2(&mut self) -> CNT_PAUSE_U_W<'_, CTRL_SPEC> {
        CNT_PAUSE_U_W::new(self, 5)
    }
    #[doc = "Bit 7 - Set this bit to pause unit3's counter."]
    #[inline(always)]
    pub fn cnt_pause_u3(&mut self) -> CNT_PAUSE_U_W<'_, CTRL_SPEC> {
        CNT_PAUSE_U_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures this bit to enable unit 0's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u0(&mut self) -> DALTA_CHANGE_EN_U0_W<'_, CTRL_SPEC> {
        DALTA_CHANGE_EN_U0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures this bit to enable unit 1's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u1(&mut self) -> DALTA_CHANGE_EN_U1_W<'_, CTRL_SPEC> {
        DALTA_CHANGE_EN_U1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures this bit to enable unit 2's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u2(&mut self) -> DALTA_CHANGE_EN_U2_W<'_, CTRL_SPEC> {
        DALTA_CHANGE_EN_U2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures this bit to enable unit 3's step comparator."]
    #[inline(always)]
    pub fn dalta_change_en_u3(&mut self) -> DALTA_CHANGE_EN_U3_W<'_, CTRL_SPEC> {
        DALTA_CHANGE_EN_U3_W::new(self, 11)
    }
    #[doc = "Bit 16 - The registers clock gate enable signal of PCNT module. 1: the registers can be read and written by application. 0: the registers can not be read or written by application"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CTRL_SPEC> {
        CLK_EN_W::new(self, 16)
    }
}
#[doc = "Control register for all counters\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
