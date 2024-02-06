#[doc = "Register `INT_ST0` reader"]
pub type R = crate::R<INT_ST0_SPEC>;
#[doc = "Field `ACK_WITH_ERR_0` reader - NA"]
pub type ACK_WITH_ERR_0_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_1` reader - NA"]
pub type ACK_WITH_ERR_1_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_2` reader - NA"]
pub type ACK_WITH_ERR_2_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_3` reader - NA"]
pub type ACK_WITH_ERR_3_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_4` reader - NA"]
pub type ACK_WITH_ERR_4_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_5` reader - NA"]
pub type ACK_WITH_ERR_5_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_6` reader - NA"]
pub type ACK_WITH_ERR_6_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_7` reader - NA"]
pub type ACK_WITH_ERR_7_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_8` reader - NA"]
pub type ACK_WITH_ERR_8_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_9` reader - NA"]
pub type ACK_WITH_ERR_9_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_10` reader - NA"]
pub type ACK_WITH_ERR_10_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_11` reader - NA"]
pub type ACK_WITH_ERR_11_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_12` reader - NA"]
pub type ACK_WITH_ERR_12_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_13` reader - NA"]
pub type ACK_WITH_ERR_13_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_14` reader - NA"]
pub type ACK_WITH_ERR_14_R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_15` reader - NA"]
pub type ACK_WITH_ERR_15_R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_0` reader - NA"]
pub type DPHY_ERRORS_0_R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_1` reader - NA"]
pub type DPHY_ERRORS_1_R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_2` reader - NA"]
pub type DPHY_ERRORS_2_R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_3` reader - NA"]
pub type DPHY_ERRORS_3_R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_4` reader - NA"]
pub type DPHY_ERRORS_4_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ack_with_err_0(&self) -> ACK_WITH_ERR_0_R {
        ACK_WITH_ERR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ack_with_err_1(&self) -> ACK_WITH_ERR_1_R {
        ACK_WITH_ERR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ack_with_err_2(&self) -> ACK_WITH_ERR_2_R {
        ACK_WITH_ERR_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ack_with_err_3(&self) -> ACK_WITH_ERR_3_R {
        ACK_WITH_ERR_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ack_with_err_4(&self) -> ACK_WITH_ERR_4_R {
        ACK_WITH_ERR_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ack_with_err_5(&self) -> ACK_WITH_ERR_5_R {
        ACK_WITH_ERR_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ack_with_err_6(&self) -> ACK_WITH_ERR_6_R {
        ACK_WITH_ERR_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn ack_with_err_7(&self) -> ACK_WITH_ERR_7_R {
        ACK_WITH_ERR_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn ack_with_err_8(&self) -> ACK_WITH_ERR_8_R {
        ACK_WITH_ERR_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn ack_with_err_9(&self) -> ACK_WITH_ERR_9_R {
        ACK_WITH_ERR_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn ack_with_err_10(&self) -> ACK_WITH_ERR_10_R {
        ACK_WITH_ERR_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ack_with_err_11(&self) -> ACK_WITH_ERR_11_R {
        ACK_WITH_ERR_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ack_with_err_12(&self) -> ACK_WITH_ERR_12_R {
        ACK_WITH_ERR_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ack_with_err_13(&self) -> ACK_WITH_ERR_13_R {
        ACK_WITH_ERR_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ack_with_err_14(&self) -> ACK_WITH_ERR_14_R {
        ACK_WITH_ERR_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ack_with_err_15(&self) -> ACK_WITH_ERR_15_R {
        ACK_WITH_ERR_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dphy_errors_0(&self) -> DPHY_ERRORS_0_R {
        DPHY_ERRORS_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dphy_errors_1(&self) -> DPHY_ERRORS_1_R {
        DPHY_ERRORS_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn dphy_errors_2(&self) -> DPHY_ERRORS_2_R {
        DPHY_ERRORS_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dphy_errors_3(&self) -> DPHY_ERRORS_3_R {
        DPHY_ERRORS_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn dphy_errors_4(&self) -> DPHY_ERRORS_4_R {
        DPHY_ERRORS_4_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST0")
            .field(
                "ack_with_err_0",
                &format_args!("{}", self.ack_with_err_0().bit()),
            )
            .field(
                "ack_with_err_1",
                &format_args!("{}", self.ack_with_err_1().bit()),
            )
            .field(
                "ack_with_err_2",
                &format_args!("{}", self.ack_with_err_2().bit()),
            )
            .field(
                "ack_with_err_3",
                &format_args!("{}", self.ack_with_err_3().bit()),
            )
            .field(
                "ack_with_err_4",
                &format_args!("{}", self.ack_with_err_4().bit()),
            )
            .field(
                "ack_with_err_5",
                &format_args!("{}", self.ack_with_err_5().bit()),
            )
            .field(
                "ack_with_err_6",
                &format_args!("{}", self.ack_with_err_6().bit()),
            )
            .field(
                "ack_with_err_7",
                &format_args!("{}", self.ack_with_err_7().bit()),
            )
            .field(
                "ack_with_err_8",
                &format_args!("{}", self.ack_with_err_8().bit()),
            )
            .field(
                "ack_with_err_9",
                &format_args!("{}", self.ack_with_err_9().bit()),
            )
            .field(
                "ack_with_err_10",
                &format_args!("{}", self.ack_with_err_10().bit()),
            )
            .field(
                "ack_with_err_11",
                &format_args!("{}", self.ack_with_err_11().bit()),
            )
            .field(
                "ack_with_err_12",
                &format_args!("{}", self.ack_with_err_12().bit()),
            )
            .field(
                "ack_with_err_13",
                &format_args!("{}", self.ack_with_err_13().bit()),
            )
            .field(
                "ack_with_err_14",
                &format_args!("{}", self.ack_with_err_14().bit()),
            )
            .field(
                "ack_with_err_15",
                &format_args!("{}", self.ack_with_err_15().bit()),
            )
            .field(
                "dphy_errors_0",
                &format_args!("{}", self.dphy_errors_0().bit()),
            )
            .field(
                "dphy_errors_1",
                &format_args!("{}", self.dphy_errors_1().bit()),
            )
            .field(
                "dphy_errors_2",
                &format_args!("{}", self.dphy_errors_2().bit()),
            )
            .field(
                "dphy_errors_3",
                &format_args!("{}", self.dphy_errors_3().bit()),
            )
            .field(
                "dphy_errors_4",
                &format_args!("{}", self.dphy_errors_4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST0_SPEC;
impl crate::RegisterSpec for INT_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st0::R`](R) reader structure"]
impl crate::Readable for INT_ST0_SPEC {}
#[doc = "`reset()` method sets INT_ST0 to value 0"]
impl crate::Resettable for INT_ST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
