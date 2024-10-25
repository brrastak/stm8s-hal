#[doc = "Register `DDR` reader"]
pub type R = crate::R<DDR_SPEC>;
#[doc = "Register `DDR` writer"]
pub type W = crate::W<DDR_SPEC>;
#[doc = "Field `DDR0` reader - "]
pub type DDR0_R = crate::BitReader;
#[doc = "Field `DDR0` writer - "]
pub type DDR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1` reader - "]
pub type DDR1_R = crate::BitReader;
#[doc = "Field `DDR1` writer - "]
pub type DDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR2` reader - "]
pub type DDR2_R = crate::BitReader;
#[doc = "Field `DDR2` writer - "]
pub type DDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR3` reader - "]
pub type DDR3_R = crate::BitReader;
#[doc = "Field `DDR3` writer - "]
pub type DDR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR4` reader - "]
pub type DDR4_R = crate::BitReader;
#[doc = "Field `DDR4` writer - "]
pub type DDR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR5` reader - "]
pub type DDR5_R = crate::BitReader;
#[doc = "Field `DDR5` writer - "]
pub type DDR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR6` reader - "]
pub type DDR6_R = crate::BitReader;
#[doc = "Field `DDR6` writer - "]
pub type DDR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR7` reader - "]
pub type DDR7_R = crate::BitReader;
#[doc = "Field `DDR7` writer - "]
pub type DDR7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ddr0(&self) -> DDR0_R {
        DDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ddr1(&self) -> DDR1_R {
        DDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ddr2(&self) -> DDR2_R {
        DDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ddr4(&self) -> DDR4_R {
        DDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ddr5(&self) -> DDR5_R {
        DDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ddr6(&self) -> DDR6_R {
        DDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ddr7(&self) -> DDR7_R {
        DDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0(&mut self) -> DDR0_W<DDR_SPEC> {
        DDR0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1(&mut self) -> DDR1_W<DDR_SPEC> {
        DDR1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2(&mut self) -> DDR2_W<DDR_SPEC> {
        DDR2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ddr3(&mut self) -> DDR3_W<DDR_SPEC> {
        DDR3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4(&mut self) -> DDR4_W<DDR_SPEC> {
        DDR4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ddr5(&mut self) -> DDR5_W<DDR_SPEC> {
        DDR5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ddr6(&mut self) -> DDR6_W<DDR_SPEC> {
        DDR6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ddr7(&mut self) -> DDR7_W<DDR_SPEC> {
        DDR7_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port A data direction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddr::R`](R) reader structure"]
impl crate::Readable for DDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddr::W`](W) writer structure"]
impl crate::Writable for DDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDR to value 0"]
impl crate::Resettable for DDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
